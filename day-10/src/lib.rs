#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
enum Dir {
    N,
    S,
    E,
    W,
}

pub fn calculate_steps(board: Vec<&str>) -> i32 {
    let start = find_start_position(board.clone()).unwrap();

    let longest_path = [
        walk(board.clone(), (start, Dir::N), 0),
        walk(board.clone(), (start, Dir::S), 0),
        walk(board.clone(), (start, Dir::E), 0),
        walk(board.clone(), (start, Dir::W), 0),
    ];

    *longest_path.iter().max().unwrap()
}

pub fn calculate_number_of_enclosing_points(board: Vec<&str>) -> i32 {
    let path = find_longest_path(board.clone());
    let area = shoelace_formula(path.clone());

    area + 1 - (path.len() / 2) as i32
}

fn find_longest_path(board: Vec<&str>) -> Vec<(i32, i32)> {
    let start = find_start_position(board.clone()).unwrap();

    let longest_path = [
        find_path(board.clone(), (start, Dir::N)),
        find_path(board.clone(), (start, Dir::S)),
        find_path(board.clone(), (start, Dir::E)),
        find_path(board.clone(), (start, Dir::W)),
    ];

    longest_path
        .iter()
        .max_by_key(|path| path.clone().len())
        .unwrap()
        .clone()
}

fn shoelace_formula(points: Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += points[i].0 * points[j].1 - points[j].0 * points[i].1;
    }

    sum.abs() / 2
}

fn find_start_position(board: Vec<&str>) -> Option<(i32, i32)> {
    board
        .into_iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, tile)| *tile == 'S')
                .map(move |(col, _)| (row as i32, col as i32))
        })
        .next()
}

fn next_position((row, col): (i32, i32), tile: char, dir: Dir) -> Option<((i32, i32), Dir)> {
    match (tile, dir) {
        ('|', Dir::S) => Some(((row + 1, col), Dir::S)),
        ('|', Dir::N) => Some(((row - 1, col), Dir::N)),
        ('-', Dir::E) => Some(((row, col + 1), Dir::E)),
        ('-', Dir::W) => Some(((row, col - 1), Dir::W)),
        ('L', Dir::S) => Some(((row, col + 1), Dir::E)),
        ('L', Dir::W) => Some(((row - 1, col), Dir::N)),
        ('J', Dir::S) => Some(((row, col - 1), Dir::W)),
        ('J', Dir::E) => Some(((row - 1, col), Dir::N)),
        ('7', Dir::N) => Some(((row, col - 1), Dir::W)),
        ('7', Dir::E) => Some(((row + 1, col), Dir::S)),
        ('F', Dir::N) => Some(((row, col + 1), Dir::E)),
        ('F', Dir::W) => Some(((row + 1, col), Dir::S)),
        ('S', Dir::W) => Some(((row, col + 1), Dir::W)),
        ('S', Dir::E) => Some(((row, col - 1), Dir::E)),
        ('S', Dir::N) => Some(((row - 1, col), Dir::N)),
        ('S', Dir::S) => Some(((row + 1, col), Dir::S)),
        _ => None,
    }
}

fn walk(board: Vec<&str>, pos: ((i32, i32), Dir), steps: i32) -> i32 {
    let (row, col) = pos.0;

    if row < 0 || col < 0 {
        return steps;
    }

    let tile = get_tile(board.clone(), pos.0);
    let next_pos = tile.and_then(|tile| next_position(pos.0, tile, pos.1));

    next_pos
        .map(|next_pos| {
            let next_tile = get_tile(board.clone(), next_pos.0);

            match next_tile {
                Some('S') => (steps + 1) / 2,
                Some(_) => walk(board.clone(), next_pos, steps + 1),
                None => steps + 1,
            }
        })
        .unwrap_or(steps)
}

fn find_path(board: Vec<&str>, start: ((i32, i32), Dir)) -> Vec<(i32, i32)> {
    let mut path = vec![];
    let mut pos = start;

    loop {
        let (row, col) = pos.0;

        if row < 0 || col < 0 {
            break;
        }

        let tile = get_tile(board.clone(), pos.0);
        let next_pos = tile.and_then(|tile| next_position(pos.0, tile, pos.1));

        match next_pos {
            Some(next_pos) => {
                let next_tile = get_tile(board.clone(), next_pos.0);
                if next_tile == Some('S') {
                    path.push(next_pos.0);
                    break;
                } else {
                    path.push(pos.0);
                    pos = next_pos;
                }
            }
            None => {
                path.push(pos.0);
                break;
            }
        }
    }

    path
}

fn get_tile(board: Vec<&str>, (row, col): (i32, i32)) -> Option<char> {
    board[row as usize].chars().nth(col as usize)
}

#[test]
fn should_count_number_of_steps_from_start() {
    let board = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];

    let steps = calculate_steps(board);

    assert_eq!(steps, 8);
}

#[test]
fn should_find_the_longest_path_from_start() {
    let board = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];

    let path = find_longest_path(board);

    assert_eq!(
        path,
        vec![
            (2, 0),
            (3, 0),
            (4, 0),
            (4, 1),
            (3, 1),
            (3, 2),
            (3, 3),
            (3, 4),
            (2, 4),
            (2, 3),
            (1, 3),
            (0, 3),
            (0, 2),
            (1, 2),
            (1, 1),
            (2, 0),
        ]
    );
}

#[test]
fn should_calculate_next_position_for_north_to_east() {
    let position = (1, 2);

    let next_position = next_position(position, 'L', Dir::S);

    assert_eq!(next_position, Some(((1, 3), Dir::E)));
}

#[test]
fn should_calculate_next_position_for_north_to_south() {
    let position = (1, 2);

    let next_position = next_position(position, '|', Dir::S);

    assert_eq!(next_position, Some(((2, 2), Dir::S)));
}

#[test]
fn should_calculate_next_position_for_south_to_north() {
    let position = (1, 2);

    let next_position = next_position(position, '|', Dir::N);

    assert_eq!(next_position, Some(((0, 2), Dir::N)));
}

#[test]
fn should_calculate_next_position_for_east_to_west() {
    let position = (1, 2);

    let next_position = next_position(position, '-', Dir::E);

    assert_eq!(next_position, Some(((1, 3), Dir::E)));
}

#[test]
fn should_find_start_position() {
    let board = vec![".....", ".S-7.", ".|.|.", ".L-J.", "....."];

    let start_position = find_start_position(board);

    assert_eq!(start_position, Some((1, 1)));
}

#[test]
fn should_return_none_if_no_start_position_is_availabe() {
    let board = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];

    let start_position = find_start_position(board);

    assert_eq!(start_position, Some((2, 0)));
}

#[test]
fn should_calcualte_the_area_of_a_pentagon() {
    let points = vec![(1, 6), (3, 1), (7, 2), (4, 4), (8, 5)];

    let area = shoelace_formula(points);

    assert_eq!(area, 16);
}
