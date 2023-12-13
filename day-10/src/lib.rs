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

fn get_tile(board: Vec<&str>, (row, col): (i32, i32)) -> Option<char> {
    board[row as usize].chars().nth(col as usize)
}

#[test]
fn should_number_of_steps_from_start() {
    let board = vec!["..F7.", ".FJ|.", "SJ.L7", "|F--J", "LJ..."];

    let steps = calculate_steps(board);

    assert_eq!(steps, 8);
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
