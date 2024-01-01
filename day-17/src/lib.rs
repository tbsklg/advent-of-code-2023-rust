use std::{collections::HashMap};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}

type Coordinate = (i64, i64);
type Position = (Coordinate, Dir, u8);

pub fn heat_loss(input: Vec<&str>) -> u32 {
    djikstra(&input)
}

fn djikstra(input: &Vec<&str>) -> u32 {
    let target = ((input.len() - 1) as i64, (input[0].len() - 1) as i64);

    let mut queue = vec![((0, 1), Dir::East, 0), ((1, 0), Dir::South, 0)];

    let mut visited = vec![(0, 0)];
    let mut distances: HashMap<_, u32> = queue.iter().map(|x| (x.0, heat(input, &x.0))).collect();

    while !queue.is_empty() {
        let pos @ (coord, _, _) = queue.remove(0);

        if visited.contains(&coord) {
            continue;
        }

        visited.push(coord);

        let neighbours = next_positions(input, &pos);

        for neighbour in neighbours {
            let heat = heat(input, &neighbour.0);
            let new_distance = distances.get(&coord).unwrap() + heat;

            if distances.contains_key(&neighbour.0) {
                let current_distance = distances.get(&neighbour.0).unwrap();

                if new_distance < *current_distance {
                    distances.insert(neighbour.0, new_distance);
                }
            } else {
                distances.insert(neighbour.0, new_distance);
            }

            queue.push(neighbour);
        }
    }

    *distances.get(&target).unwrap()
}

fn next_positions(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let (_, _, walk_count) = pos;

    let directions = change_direction(input, pos);

    if *walk_count + 1 > 3 {
        directions
    } else {
        vec![next_position(input, pos.clone())]
            .into_iter()
            .flatten()
            .chain(directions.into_iter())
            .collect()
    }
}

fn next_position(input: &Vec<&str>, pos: Position) -> Option<Position> {
    let ((r, c), dir, walk_count) = pos;

    match dir {
        Dir::North => position(input, ((r - 1, c), dir, walk_count + 1)),
        Dir::South => position(input, ((r + 1, c), dir, walk_count + 1)),
        Dir::East => position(input, ((r, c + 1), dir, walk_count + 1)),
        Dir::West => position(input, ((r, c - 1), dir, walk_count + 1)),
    }
}

fn change_direction(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let (_, dir, _) = &pos;

    match dir {
        Dir::North | Dir::South => horizontal_positions(input, pos),
        Dir::East | Dir::West => vertical_positions(input, pos),
    }
}

fn heat(input: &Vec<&str>, coord: &Coordinate) -> u32 {
    let (r, c) = coord;

    let heat = input
        .get(*r as usize)
        .unwrap()
        .chars()
        .nth(*c as usize)
        .unwrap();

    heat.to_digit(10).unwrap()
}

fn vertical_positions(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let ((r, c), _, _) = &pos;

    let north = position(input, ((*r - 1, *c), Dir::North, 0));
    let south = position(input, ((*r + 1, *c), Dir::South, 0));

    vec![north, south]
        .into_iter()
        .flatten()
        .collect()
}

fn horizontal_positions(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let ((r, c), _, _) = &pos;

    let east = position(input, ((*r, *c + 1), Dir::East, 0));
    let west = position(input, ((*r, *c - 1), Dir::West, 0));

    vec![east, west]
        .into_iter()
        .flatten()
        .collect()
}

fn position(input: &Vec<&str>, pos: Position) -> Option<Position> {
    match in_bound(input, pos.clone()) {
        true => Some(pos),
        false => None,
    }
}

fn in_bound(input: &Vec<&str>, pos: Position) -> bool {
    let ((r, c), _, _) = pos;

    r >= 0 && r < input.len() as i64 && c >= 0 && c < input[0].len() as i64
}
