use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    iter::Map,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Ord, PartialOrd, Hash)]
enum Dir {
    North,
    South,
    West,
    East,
}

type Coordinate = (i64, i64);
type Position = (Coordinate, Dir);

pub fn heat_loss(input: Vec<&str>) -> usize {
    djikstra(&input)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Position,
    steps: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct StateKey {
    position: Position,
    steps: u8,
}

impl From<State> for StateKey {
    fn from(value: State) -> Self {
        Self {
            position: value.position,
            steps: value.steps,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn djikstra(input: &Vec<&str>) -> usize {
    let target = ((input.len() - 1) as i64, (input[0].len() - 1) as i64);

    let mut heap = BinaryHeap::new();
    let east_start = State {
        cost: 0,
        position: ((0, 0), Dir::East),
        steps: 0,
    };

    let south_start = State {
        cost: 0,
        position: ((0, 0), Dir::South),
        steps: 0,
    };

    heap.push(east_start);
    heap.push(south_start);

    let mut distances: HashMap<StateKey, usize> = HashMap::new();
    distances.insert(east_start.into(), 0);
    distances.insert(south_start.into(), 0);

    while let Some(
        state @ State {
            cost,
            position,
            steps,
        },
    ) = heap.pop()
    {
        if position.0 == target {
            return cost;
        }

        if distances.contains_key(&state.into()) && distances.get(&state.into()).unwrap() < &cost {
            continue;
        }

        let neighbours = next_position(&input, &position);

        for neighbour in neighbours {
            let heat = heat(&input, &neighbour.0);
            let new_distance = cost + heat as usize;

            let curr_dir = position.1;
            let next_dir = neighbour.1;

            let next_state = State {
                cost: new_distance as usize,
                position: neighbour,
                steps: if curr_dir == next_dir {
                    steps + 1
                } else {
                    steps
                },
            };

            if next_state.steps > 3
                || distances.contains_key(&next_state.into())
                    && distances.get(&next_state.into()).unwrap() < &new_distance
            {
                continue;
            }

            distances.insert(next_state.into(), new_distance);
            heap.push(next_state);
        }
    }

    0
}

fn next_position(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let ((r, c), _) = pos;

    vec![
        position(&input, &((r - 1, c.clone()), Dir::North)),
        position(&input, &((r + 1, c.clone()), Dir::South)),
        position(&input, &((r.clone(), c + 1), Dir::East)),
        position(&input, &((r.clone(), c - 1), Dir::West)),
    ]
    .into_iter()
    .filter(|x| x.is_some())
    .map(|x| x.unwrap())
    .collect()
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

fn position(input: &Vec<&str>, pos: &Position) -> Option<Position> {
    match in_bound(input, pos) {
        true => Some(pos.clone()),
        false => None,
    }
}

fn in_bound(input: &Vec<&str>, pos: &Position) -> bool {
    let ((r, c), _) = pos;

    *r >= 0 && *r < input.len() as i64 && *c >= 0 && *c < input[0].len() as i64
}
