#![warn(clippy::pedantic)]

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    cube_sets: Vec<HashMap<Color, u32>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Color {
    Blue,
    Red,
    Green,
}

#[must_use]
pub fn calculate_sum_of_game_ids(input: Vec<&str>, cube_set: HashMap<Color, u32>) -> u32 {
    let games = input
        .iter()
        .filter_map(|line| create_game(line))
        .collect::<Vec<Game>>();

    sum_up_ids(&games, cube_set)
}

#[must_use]
pub fn calculate_sum_of_the_power_of_cube_sets(input: Vec<&str>) -> u32 {
    let games = input
        .iter()
        .filter_map(|line| create_game(line))
        .collect::<Vec<Game>>();

    sum_up_the_power_of_ids(&games)
}

impl Game {
    fn from(id: u32, cube_sets: Vec<HashMap<Color, u32>>) -> Self {
        Self { id, cube_sets }
    }

    fn is_possible(&self, cube_set: &HashMap<Color, u32>) -> bool {
        self.cube_sets.iter().all(|set| {
            set.iter().all(|(color, quantity)| {
                cube_set
                    .get(color)
                    .map_or(false, |cube_quantity| cube_quantity >= quantity)
            })
        })
    }

    fn fewest_number_of_cubes(&self) -> HashMap<Color, u32> {
        self.cube_sets
            .iter()
            .fold(HashMap::new(), |mut fewest, cube_set| {
                for (color, quantity) in cube_set {
                    let current_quantity = fewest.get(color);
                    match current_quantity {
                        Some(current_quantity) => {
                            if quantity > current_quantity {
                                fewest.insert(*color, *quantity);
                            }
                        }
                        None => {
                            fewest.insert(*color, *quantity);
                        }
                    }
                }
                fewest
            })
    }
}

fn sum_up_the_power_of_ids(games: &[Game]) -> u32 {
    games
        .iter()
        .map(Game::fewest_number_of_cubes)
        .map(|cube_set| cube_set.values().product::<u32>())
        .sum()
}

fn sum_up_ids(games: &[Game], cube_set: HashMap<Color, u32>) -> u32 {
    games
        .iter()
        .filter(|game| game.is_possible(&cube_set))
        .map(|game| game.id)
        .sum()
}

fn create_game(input: &str) -> Option<Game> {
    let mut split = input.split(": ");

    let id = split.next().and_then(extract_id);

    let cube_sets = split.next().map(create_cube_sets);

    match (id, cube_sets) {
        (Some(id), Some(cube_sets)) => Some(Game::from(id, cube_sets)),
        _ => None,
    }
}

fn create_cube_sets(input: &str) -> Vec<HashMap<Color, u32>> {
    input.split("; ").map(create_cube_set).collect()
}

fn create_cube_set(input: &str) -> HashMap<Color, u32> {
    input
        .split(", ")
        .fold(HashMap::new(), |mut cube_set, cube| {
            if let Some((color, quantity)) = extract_color_and_quantity(cube) {
                cube_set.insert(color, quantity);
            }
            cube_set
        })
}

fn extract_color_and_quantity(input: &str) -> Option<(Color, u32)> {
    let mut split = input.split(' ');

    let quantity = split
        .next()
        .and_then(|quantity| quantity.parse::<u32>().ok());

    let color = split.next().and_then(map_color);

    match (color, quantity) {
        (Some(color), Some(quantity)) => Some((color, quantity)),
        _ => None,
    }
}

fn map_color(color: &str) -> Option<Color> {
    match color {
        "blue" => Some(Color::Blue),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        _ => None,
    }
}

fn extract_id(input: &str) -> Option<u32> {
    input
        .split(' ')
        .last()
        .and_then(|id| id.parse::<u32>().ok())
}

#[test]
fn should_extract_id_with_one_digit() {
    let input = "Game 1";

    let id = extract_id(input);

    assert_eq!(id, Some(1));
}

#[test]
fn should_extract_id_with_multiple_digits() {
    let input = "Game 12";

    let id = extract_id(input);

    assert_eq!(id, Some(12));
}

#[test]
fn should_extract_id_with_no_id() {
    let input = "Game";

    let id = extract_id(input);

    assert_eq!(id, None);
}

#[test]
fn should_extract_id_with_no_id_and_space() {
    let input = "Game ";

    let id = extract_id(input);

    assert_eq!(id, None);
}

#[test]
fn should_extract_id_with_no_number_provided() {
    let input = "Game x";

    let id = extract_id(input);

    assert_eq!(id, None);
}

#[test]
fn should_extract_color_and_quantity() {
    let input = "3 blue";

    let color_and_quantity = extract_color_and_quantity(input);

    assert_eq!(color_and_quantity, Some((Color::Blue, 3)));
}

#[test]
fn should_return_none_for_unknown_color() {
    let input = "2 brown";

    let color_and_quantity = extract_color_and_quantity(input);

    assert_eq!(color_and_quantity, None);
}

#[test]
fn should_create_set_from_cubes() {
    let input = "3 blue, 4 red";

    let cube_set = create_cube_set(input);

    assert_eq!(cube_set, HashMap::from([(Color::Blue, 3), (Color::Red, 4)]));
}

#[test]
fn should_create_a_list_of_cube_sets() {
    let input = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    let cube_sets = create_cube_sets(input);

    assert_eq!(
        cube_sets,
        vec![
            HashMap::from([(Color::Blue, 3), (Color::Red, 4)]),
            HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
            HashMap::from([(Color::Green, 2)])
        ]
    );
}

#[test]
fn should_create_a_game() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    let game = create_game(input);

    assert_eq!(
        game,
        Some(Game {
            id: 1,
            cube_sets: vec![
                HashMap::from([(Color::Blue, 3), (Color::Red, 4)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
                HashMap::from([(Color::Green, 2)])
            ]
        })
    );
}

#[test]
fn should_check_if_game_is_possible() {
    let game = Game {
        id: 1,
        cube_sets: vec![
            HashMap::from([(Color::Blue, 3), (Color::Red, 4)]),
            HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
            HashMap::from([(Color::Green, 2)]),
        ],
    };

    let is_possible = game.is_possible(&HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 14),
    ]));

    assert!(is_possible);
}

#[test]
fn should_check_if_game_is_not_possible() {
    let game = Game {
        id: 1,
        cube_sets: vec![
            HashMap::from([(Color::Green, 8), (Color::Blue, 6), (Color::Red, 20)]),
            HashMap::from([(Color::Blue, 5), (Color::Red, 4), (Color::Green, 13)]),
            HashMap::from([(Color::Green, 5)]),
        ],
    };

    let is_possible = game.is_possible(&HashMap::from([
        (Color::Red, 12),
        (Color::Green, 13),
        (Color::Blue, 13),
    ]));

    assert!(!is_possible);
}

#[test]
fn should_sum_up_ids_for_possible_games() {
    let games = [
        Game::from(
            1,
            vec![
                HashMap::from([(Color::Blue, 3), (Color::Red, 4)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
                HashMap::from([(Color::Green, 2)]),
            ],
        ),
        Game::from(
            2,
            vec![
                HashMap::from([(Color::Blue, 1), (Color::Green, 2)]),
                HashMap::from([(Color::Green, 3), (Color::Blue, 4), (Color::Red, 1)]),
                HashMap::from([(Color::Green, 1), (Color::Blue, 1)]),
            ],
        ),
        Game::from(
            3,
            vec![
                HashMap::from([(Color::Blue, 6), (Color::Red, 20), (Color::Green, 8)]),
                HashMap::from([(Color::Red, 4), (Color::Green, 13), (Color::Blue, 5)]),
                HashMap::from([(Color::Green, 5), (Color::Red, 1)]),
            ],
        ),
        Game::from(
            4,
            vec![
                HashMap::from([(Color::Green, 1), (Color::Blue, 6), (Color::Red, 3)]),
                HashMap::from([(Color::Red, 6), (Color::Green, 3)]),
                HashMap::from([(Color::Green, 3), (Color::Red, 14), (Color::Blue, 15)]),
            ],
        ),
        Game::from(
            5,
            vec![
                HashMap::from([(Color::Blue, 1), (Color::Red, 6), (Color::Green, 3)]),
                HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 2)]),
            ],
        ),
    ];

    let result = sum_up_ids(
        &games,
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]),
    );

    assert_eq!(result, 8);
}

#[test]
fn should_calculate_fewest_number_of_cubes() {
    let input = Game::from(
        1,
        vec![
            HashMap::from([(Color::Blue, 3), (Color::Red, 4)]),
            HashMap::from([(Color::Red, 1), (Color::Green, 2), (Color::Blue, 6)]),
            HashMap::from([(Color::Green, 2)]),
        ],
    );

    let fewest_possible = input.fewest_number_of_cubes();

    assert_eq!(
        fewest_possible,
        HashMap::from([(Color::Red, 4), (Color::Green, 2), (Color::Blue, 6)])
    );
}
