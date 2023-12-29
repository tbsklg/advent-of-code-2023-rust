use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

pub fn tiles(input: Vec<&str>) -> u64 {
    let mut pos: Vec<((i32, i32), Dir)> = vec![((0, 0), Dir::Right)];
    let mut visited: HashSet<((i32, i32), Dir)> = HashSet::new();
    let mut energized: HashSet<(i32, i32)> = HashSet::new();

    while !pos.is_empty() {
        let ((r, c), dir) = pos.remove(0);

        let curr = input
            .get(r as usize)
            .and_then(|row| row.chars().nth(c as usize));

        if curr.is_none() {
            break;
        }

        let next = next_pos(((r, c), dir), curr.unwrap())
            .into_iter()
            .filter(|((r, c), _)| in_bounds(input.clone(), *r, *c))
            .filter(|((r, c), dir)| !visited.contains(&((*r, *c), *dir)))
            .collect::<Vec<_>>();

        visited.extend(next.clone());
        energized.extend(next.iter().map(|((r, c), _)| (*r, *c)));

        pos = [next, pos].concat();
    }

    energized.len() as u64
}

fn in_bounds(input: Vec<&str>, r: i32, c: i32) -> bool {
    r >= 0 && c >= 0 && r < input.len() as i32 && c < input[0].len() as i32
}

fn next_pos(dir: ((i32, i32), Dir), sign: char) -> Vec<((i32, i32), Dir)> {
    let ((r, c), dir) = dir;

    match (dir, sign) {
        (Dir::Up, '/') => vec![((r, c + 1), Dir::Right)],
        (Dir::Down, '/') => vec![((r, c - 1), Dir::Left)],
        (Dir::Right, '/') => vec![((r - 1, c), Dir::Up)],
        (Dir::Left, '/') => vec![((r + 1, c), Dir::Down)],
        (Dir::Up, '\\') => vec![((r, c - 1), Dir::Left)],
        (Dir::Down, '\\') => vec![((r, c + 1), Dir::Right)],
        (Dir::Right, '\\') => vec![((r + 1, c), Dir::Down)],
        (Dir::Left, '\\') => vec![((r - 1, c), Dir::Up)],
        (Dir::Up, '|') => vec![((r - 1, c), dir)],
        (Dir::Down, '|') => vec![((r + 1, c), dir)],
        (Dir::Right, '|') => vec![((r + 1, c), Dir::Down), ((r - 1, c), Dir::Up)],
        (Dir::Left, '|') => vec![((r + 1, c), Dir::Down), ((r - 1, c), Dir::Up)],
        (Dir::Up, '-') => vec![((r, c - 1), Dir::Left), ((r, c + 1), Dir::Right)],
        (Dir::Down, '-') => vec![((r, c - 1), Dir::Left), ((r, c + 1), Dir::Right)],
        (Dir::Right, '-') => vec![((r, c + 1), dir)],
        (Dir::Left, '-') => vec![((r, c - 1), dir)],
        (Dir::Up, '.') => vec![((r - 1, c), dir)],
        (Dir::Down, '.') => vec![((r + 1, c), dir)],
        (Dir::Right, '.') => vec![((r, c + 1), dir)],
        (Dir::Left, '.') => vec![((r, c - 1), dir)],
        _ => vec![],
    }
}
