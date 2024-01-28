use std::collections::VecDeque;

pub fn hello(input: Vec<&str>) -> usize {
    walk(input)
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
            Dir::East => Dir::West,
        }
    }
}

type Position = ((usize, usize), Dir);

fn walk(trails: Vec<&str>) -> usize {
    let start = ((0, 1), Dir::South);

    assert!(trails[0].chars().nth(1).unwrap() == '.');

    let end = (trails.len() - 1, trails[0].len() - 2);

    let mut queue = VecDeque::from(
        neighbours(&trails, &start)
            .iter()
            .map(|p| (*p, 1))
            .collect::<Vec<(Position, usize)>>(),
    );

    let mut hikes = vec![];

    while let Some(pos) = queue.pop_front() {
        if pos.0 .0 == end {
            hikes.push(pos.1);
            continue;
        }

        let next = neighbours(&trails, &pos.0);
        for n in next {
            queue.push_back((n, pos.1 + 1));
        }
    }

    *hikes.iter().max().unwrap()
}

fn neighbours(trails: &Vec<&str>, ((r, c), d): &Position) -> Vec<Position> {
    slope(trails, &((*r, *c), *d))
        .map(|p| vec![p])
        .unwrap_or(vec![
            ((*r, *c + 1), Dir::East),
            ((*r, c.saturating_sub(1)), Dir::West),
            ((*r + 1, *c), Dir::South),
            ((r.saturating_sub(1), *c), Dir::North),
        ])
        .into_iter()
        .filter(|(_, n)| *n != d.opposite())
        .filter(|((r, c), _)| trails[*r].chars().nth(*c).unwrap() != '#')
        .collect()
}

fn slope(trails: &Vec<&str>, ((r, c), _): &Position) -> Option<Position> {
    match trails[*r].chars().nth(*c).unwrap() {
        '>' => Some(((*r, *c + 1), Dir::East)),
        '<' => Some(((*r, c.saturating_sub(1)), Dir::West)),
        'v' => Some(((*r + 1, *c), Dir::South)),
        '^' => Some(((r.saturating_sub(1), *c), Dir::North)),
        _ => None,
    }
}
