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
fn should_expand_universe_by_row() {
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

    let expanded = expand_by_row(input, 3);

    assert_eq!(
        expanded,
        vec![
            "...#......",
            ".......#..",
            "#.........",
            "..........",
            "..........",
            "......#...",
            ".#........",
            ".........#",
            "..........",
            ".......#..",
            "#...#.....",
        ]
    );
}

#[test]
fn should_expand_universe_by_cols() {
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

    let expanded = expand_by_cols(input, vec![2, 5, 8]);

    assert_eq!(
        expanded,
        vec![
            "....#........",
            ".........#...",
            "#............",
            ".............",
            "........#....",
            ".#...........",
            "............#",
            ".............",
            ".........#...",
            "#....#.......",
        ]
    );
}

#[test]
fn should_expand_universe() {
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

    let expanded = expand_universe(input);

    assert_eq!(
        expanded,
        vec![
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
        ]
    );
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

    let shortest_distance = find_shortest_path_between_galaxies(expanded_universe);

    assert_eq!(shortest_distance, 374);
}

pub fn sum_of_shortest_path_between_galaxies(input: Vec<&str>) -> usize {
    let input = input.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    let expanded_universe = expand_universe(input);

    find_shortest_path_between_galaxies(expanded_universe)
}

fn find_shortest_path_between_galaxies(input: Vec<String>) -> usize {
    let galaxies = find_galaxies(input);

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, g)| {
            galaxies
                .iter()
                .skip(i + 1)
                .map(move |g2| manhatten_distance(*g, *g2))
        })
        .sum()
}

fn manhatten_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let x = (a.0 as isize - b.0 as isize).unsigned_abs();
    let y = (a.1 as isize - b.1 as isize).unsigned_abs();

    x + y
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

fn expand_universe(input: Vec<String>) -> Vec<String> {
    let empty_cols = find_empty_cols(input.clone());
    let empty_rows = find_empty_rows(input.clone());

    let expanded_by_rows = expand_by_rows(input, empty_rows);
    expand_by_cols(expanded_by_rows, empty_cols)
}

fn expand_by_cols(input: Vec<String>, cols: Vec<usize>) -> Vec<String> {
    let cols = cols
        .iter()
        .zip(0..)
        .map(|(a, b)| a + b)
        .collect::<Vec<usize>>();

    cols.iter()
        .fold(input, |acc, c| expand_by_col(acc, *c))
}

fn expand_by_rows(input: Vec<String>, rows: Vec<usize>) -> Vec<String> {
    let rows = rows
        .iter()
        .zip(0..)
        .map(|(a, b)| a + b)
        .collect::<Vec<usize>>();

    rows.iter()
        .fold(input, |acc, r| expand_by_row(acc, *r))
}

fn expand_by_col(input: Vec<String>, col_index: usize) -> Vec<String> {
    input
        .iter()
        .map(|r| {
            let mut row = r.to_string();
            row.insert(col_index, '.');
            row
        })
        .collect()
}

fn expand_by_row(input: Vec<String>, row_index: usize) -> Vec<String> {
    let row = input.get(row_index).unwrap();

    let mut expanded = input.clone();
    expanded.insert(row_index + 1, row.clone());
    expanded
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
