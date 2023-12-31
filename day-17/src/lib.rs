use std::{collections::HashMap, iter::Map};

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
    let mut distances: HashMap<_, u32> = queue.iter().map(|x| (x.0, heat(&input, &x.0))).collect();

    while !queue.is_empty() {
        let pos @ (coord, _, _) = queue.remove(0);

        println!("{:?}", distances);
        println!("{:?}", pos);
        let distance = distances.get(&coord).unwrap().clone();

        let current_heat = heat(&input, &coord);
        let block_dist = distance + current_heat;

        if visited.contains(&coord) {
            continue;
        }

        visited.push(coord.clone());

        if distances.contains_key(&coord) {
            let curr_dist = distances.get(&coord).unwrap();

            if block_dist < *curr_dist {
                distances.insert(coord, block_dist);
            }
        } else {
            distances.insert(coord, block_dist);
        }

        queue.extend(next_positions(&input, &pos));
    }

    distances.get(&target).unwrap().clone()
}

fn next_positions(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let (_, _, walk_count) = pos;

    if *walk_count > 3 {
        return change_direction(&input, &pos);
    } else {
        return vec![next_position(&input, pos.clone())]
            .into_iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
    }
}

fn next_position(input: &Vec<&str>, pos: Position) -> Option<Position> {
    let ((r, c), dir, walk_count) = pos;

    match dir {
        Dir::North => position(&input, ((r - 1, c), dir, walk_count + 1)),
        Dir::South => position(&input, ((r + 1, c), dir, walk_count + 1)),
        Dir::East => position(&input, ((r, c + 1), dir, walk_count + 1)),
        Dir::West => position(&input, ((r, c - 1), dir, walk_count + 1)),
    }
}

fn change_direction(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let (_, dir, _) = &pos;

    match dir {
        Dir::North | Dir::South => horizontal_positions(&input, &pos),
        Dir::East | Dir::West => vertical_positions(&input, &pos),
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

    let north = position(&input, ((*r - 1, *c), Dir::North, 0));
    let south = position(&input, ((*r + 1, *c), Dir::South, 0));

    vec![north, south]
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect()
}

fn horizontal_positions(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let ((r, c), _, walk_count) = &pos;

    let east = position(&input, ((*r, *c + 1), Dir::East, 0));
    let west = position(&input, ((*r, *c - 1), Dir::West, 0));

    vec![east, west]
        .into_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
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
