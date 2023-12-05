use lazy_static::lazy_static;
use regex::Regex;

struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Game {
    fn from(input: &str) -> Self {
        let parts = SPLIT_REGEX.split(input).collect::<Vec<&str>>();
        let id = extract_id(parts[0]).unwrap();
        let winning_numbers = extract_numbers(parts[1]);
        let numbers = extract_numbers(parts[2]);

        Self {
            id,
            winning_numbers,
            numbers,
        }
    }
}

pub fn calculate_points(input: Vec<&str>) -> u32 {
    input
        .iter()
        .map(|x| Game::from(x))
        .map(|x| winners(x.winning_numbers, x.numbers))
        .map(calculate_score)
        .sum()
}

pub fn calculate_scratchcards(input: Vec<&str>) -> u32 {
    let winners: Vec<u32> = input
        .iter()
        .map(|x| Game::from(x))
        .map(|x| winners(x.winning_numbers, x.numbers).len() as u32)
        .collect();

    (0..winners.len() as u32)
        .map(|i| calculate_cards(i, 0, &winners))
        .sum()
}

fn calculate_cards(pos: u32, acc: u32, winners: &Vec<u32>) -> u32 {
    match winners.get(pos as usize) {
        Some(x) => (1..=*x)
            .map(|j| calculate_cards(pos + j, acc + x, winners))
            .fold(1, |total, x| total + x),
        None => acc,
    }
}

fn calculate_score(winners: Vec<u32>) -> u32 {
    winners
        .is_empty()
        .then_some(0)
        .unwrap_or_else(|| 2u32.pow((winners.len() - 1) as u32))
}

fn winners(winning_numbers: Vec<u32>, numbers: Vec<u32>) -> Vec<u32> {
    winning_numbers
        .into_iter()
        .filter(|x| numbers.contains(x))
        .collect()
}

fn extract_numbers(input: &str) -> Vec<u32> {
    NUMBER_REGEX
        .find_iter(input)
        .map(|x| x.as_str().parse::<u32>().unwrap())
        .collect()
}

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
    static ref SPLIT_REGEX: Regex = Regex::new(r"[|:]").unwrap();
}

fn extract_id(input: &str) -> Option<u32> {
    NUMBER_REGEX
        .find_iter(input)
        .map(|x| x.as_str().parse::<u32>().unwrap())
        .next()
}

#[test]
fn should_extract_id_from_card() {
    let input = "Game 1";

    let id = extract_id(input);

    assert_eq!(id, Some(1));
}

#[test]
fn should_extract_numbers_from_card() {
    let input = "41 48 83 86 17";

    let numbers = extract_numbers(input);

    assert_eq!(numbers, vec![41, 48, 83, 86, 17]);
}

#[test]
fn should_filter_winning_numbers() {
    let winning_numbers = vec![41, 48, 83, 86, 17];
    let numbers = vec![83, 86, 6, 31, 17, 9, 48, 53];

    let winners = winners(winning_numbers, numbers);

    assert_eq!(winners, vec![48, 83, 86, 17]);
}

#[test]
fn should_calculate_score() {
    let winners = vec![48, 83, 86, 17];

    let score = calculate_score(winners);

    assert_eq!(score, 8);
}

#[test]
fn should_create_a_game() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";

    let game = Game::from(input);

    assert_eq!(game.id, 1);
    assert_eq!(game.winning_numbers, vec![41, 48, 83, 86, 17]);
    assert_eq!(game.numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
}
