use std::cmp::{max, min};

pub fn sum_of_shortest_path_between_galaxies(input: Vec<&str>, factor: usize) -> usize {
    let input = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();

    find_shortest_path_between_galaxies(input, factor - 1)
}

fn find_shortest_path_between_galaxies(input: Vec<String>, factor: usize) -> usize {
    let galaxies = find_galaxies(input.clone());

    let empty_rows = find_empty_rows(input.clone());
    let empty_cols = find_empty_cols(input);

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g)| {
            galaxies.iter().skip(i + 1).map(|g2| {
                let rows_between = empty_rows
                    .iter()
                    .filter(|r| **r > min(g.0, g2.0) && **r < max(g.0, g2.0))
                    .count()
                    * factor;

                let cols_between = empty_cols
                    .iter()
                    .filter(|c| **c > min(g.1, g2.1) && **c < max(g.1, g2.1))
                    .count()
                    * factor;

                manhatten_distance(*g, *g2) + rows_between + cols_between
            })
        })
        .sum()
}

fn manhatten_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let rows = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let cols = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };

    rows + cols
}

fn find_galaxies(input: Vec<String>) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];

    for (row_index, row) in input.iter().enumerate() {
        for (col_index, col) in row.chars().enumerate() {
            if col == '#' {
                galaxies.push((row_index, col_index));
            }
        }
    }

    galaxies
}

fn find_empty_cols(input: Vec<String>) -> Vec<usize> {
    input[0]
        .chars()
        .enumerate()
        .filter(|(i, _)| input.iter().all(|r| r.chars().nth(*i).unwrap() == '.'))
        .map(|(i, _)| i)
        .collect()
}

fn find_empty_rows(input: Vec<String>) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .filter(|(_, r)| r.chars().all(|c| c == '.'))
        .map(|(i, _)| i)
        .collect()
}

#[test]
fn should_find_empty_rows() {
    let input = vec![
        "...#......".to_string(),
        ".......#..".to_string(),
        "#.........".to_string(),
        "..........".to_string(),
        "......#...".to_string(),
        ".#........".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        ".......#..".to_string(),
        "#...#.....".to_string(),
    ];

    let empty_rows = find_empty_rows(input);

    assert_eq!(empty_rows, vec![3, 7]);
}

#[test]
fn should_find_empty_cols() {
    let input = vec![
        "...#......".to_string(),
        ".......#..".to_string(),
        "#.........".to_string(),
        "..........".to_string(),
        "......#...".to_string(),
        ".#........".to_string(),
        ".........#".to_string(),
        "..........".to_string(),
        ".......#..".to_string(),
        "#...#.....".to_string(),
    ];

    let empty_cols = find_empty_cols(input);

    assert_eq!(empty_cols, vec![2, 5, 8]);
}

#[test]
fn should_find_galaxies() {
    let expanded_universe = vec![
        "....#........".to_string(),
        ".........#...".to_string(),
        "#............".to_string(),
        ".............".to_string(),
        ".............".to_string(),
        "........#....".to_string(),
        ".#...........".to_string(),
        "............#".to_string(),
        ".............".to_string(),
        ".............".to_string(),
        ".........#...".to_string(),
        "#....#.......".to_string(),
    ];

    let galaxies = find_galaxies(expanded_universe);

    assert_eq!(
        galaxies,
        vec![
            (0, 4),
            (1, 9),
            (2, 0),
            (5, 8),
            (6, 1),
            (7, 12),
            (10, 9),
            (11, 0),
            (11, 5),
        ]
    );
}

#[test]
fn should_find_shortest_distance_between_galaxies() {
    let expanded_universe = vec![
        "....#........".to_string(),
        ".........#...".to_string(),
        "#............".to_string(),
        ".............".to_string(),
        ".............".to_string(),
        "........#....".to_string(),
        ".#...........".to_string(),
        "............#".to_string(),
        ".............".to_string(),
        ".............".to_string(),
        ".........#...".to_string(),
        "#....#.......".to_string(),
    ];

    let shortest_distance = find_shortest_path_between_galaxies(expanded_universe, 2);

    assert_eq!(shortest_distance, 702);
}
