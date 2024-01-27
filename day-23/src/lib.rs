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
    fn opposite(self: &Self) -> Dir {
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
    let mut start = ((0, 1), Dir::South);

    assert!(trails[0].chars().nth(1).unwrap() == '.');

    let end = (trails.len() - 1, trails[0].len() - 2);
    let mut path = 0;

    let mut queue = VecDeque::from(neighbours(&trails, &start));

    while let Some(pos) = queue.pop_front() {
        println!("Pos {:?}", pos);
        if pos.0 == end {
            break;
        }

        let next = neighbours(&trails, &pos);
        for n in next {
            queue.push_back(n);
        }
    }

    4
}

fn neighbours(trails: &Vec<&str>, ((r, c), d): &Position) -> Vec<Position> {
    slope(&trails, &((*r, *c), *d))
        .map(|p| vec![p])
        .unwrap_or(vec![
            ((r.clone(), c.clone() + 1), Dir::East),
            ((r.clone(), c.saturating_sub(1)), Dir::West),
            ((r.clone() + 1, c.clone()), Dir::South),
            ((r.saturating_sub(1), c.clone()), Dir::North),
        ])
        .into_iter()
        .filter(|(_, n)| *n != d.opposite())
        .filter(|((r, c), _)| trails[*r].chars().nth(*c).unwrap() != '#')
        .collect()
}

fn slope(trails: &Vec<&str>, ((r, c), _): &Position) -> Option<Position> {
    match trails[*r].chars().nth(*c).unwrap() {
        '>' => Some(((r.clone(), c.clone() + 1), Dir::East)),
        '<' => Some(((r.clone(), c.saturating_sub(1)), Dir::West)),
        'v' => Some(((r.clone() + 1, c.clone()), Dir::South)),
        '^' => Some(((r.saturating_sub(1), c.clone()), Dir::North)),
        _ => None,
    }
}
