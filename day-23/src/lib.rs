use std::{
    cmp::max,
    collections::{HashMap, HashSet, VecDeque},
};

pub fn longest_path_slope(input: Vec<&str>) -> usize {
    walk(input)
}

pub fn longest_path(input: Vec<&str>) -> isize {
    let start = (0, 1);
    let end = (input.len() - 1, input[0].len() - 2);

    find_longest_path(start, end, &distances(input.clone(), points(input.clone())))
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
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
type Distances = HashMap<(usize, usize), HashMap<(usize, usize), isize>>;

fn points(trails: Vec<&str>) -> Vec<(usize, usize)> {
    let start = (0, 1);
    let end = (trails.len() - 1, trails[0].len() - 2);

    let mut points = vec![start, end];
    for (r, row) in trails.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == '#' {
                continue;
            }

            if next(&trails, &(r, c)).len() >= 3 {
                points.push((r, c));
            }
        }
    }

    points
}

fn distances(trails: Vec<&str>, points: Vec<(usize, usize)>) -> Distances {
    let mut distances = HashMap::<(usize, usize), HashMap<(usize, usize), isize>>::new();

    for (sr, sc) in &points {
        let mut queue = VecDeque::from([(*sr, *sc, 0)]);
        let mut seen = HashSet::from([(*sr, *sc)]);

        while let Some((nr, nc, w)) = queue.pop_front() {
            if w != 0 && points.contains(&(nr, nc)) {
                distances.entry((*sr, *sc)).or_default().insert((nr, nc), w);
                continue;
            }

            for (dr, dc) in next(&trails, &(nr, nc)) {
                if !seen.contains(&(dr, dc)) {
                    queue.push_front((dr, dc, w + 1));
                    seen.insert((dr, dc));
                }
            }
        }
    }

    distances
}

fn find_longest_path(start: (usize, usize), end: (usize, usize), distances: &Distances) -> isize {
    fn dfs(
        current: (usize, usize),
        end: (usize, usize),
        seen: &mut HashSet<(usize, usize)>,
        distances: &Distances,
    ) -> isize {
        if current == end {
            return 0;
        }

        let mut m = isize::MIN;

        seen.insert(current);

        for n in distances.get(&current).unwrap().keys() {
            if !seen.contains(n) {
                m = max(
                    m,
                    dfs(*n, end, seen, distances)
                        + distances.get(&current).unwrap().get(n).unwrap(),
                );
            }
        }

        seen.remove(&current);

        m
    }

    let seen = &mut HashSet::<(usize, usize)>::new();
    dfs(start, end, seen, distances)
}

fn walk(trails: Vec<&str>) -> usize {
    let start = ((0, 1), Dir::South);

    assert!(trails[0].chars().nth(1).unwrap() == '.');

    let end = (trails.len() - 1, trails[0].len() - 2);

    let mut queue = VecDeque::from(
        next_with_slope(&trails, &start)
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

        let next = next_with_slope(&trails, &pos.0);
        for n in next {
            queue.push_back((n, pos.1 + 1));
        }
    }

    *hikes.iter().max().unwrap()
}

fn next(trails: &Vec<&str>, (r, c): &(usize, usize)) -> Vec<(usize, usize)> {
    vec![
        ((*r, *c + 1)),
        ((*r, c.saturating_sub(1))),
        ((*r + 1, *c)),
        ((r.saturating_sub(1), *c)),
    ]
    .into_iter()
    .filter(|(nr, nc)| nr < &trails.len() && nc < &trails[0].len())
    .filter(|(nr, nc)| trails[*nr].chars().nth(*nc).unwrap() != '#')
    .collect()
}

fn next_with_slope(trails: &Vec<&str>, ((r, c), d): &Position) -> Vec<Position> {
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
