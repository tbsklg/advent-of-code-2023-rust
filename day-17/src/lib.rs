use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    North,
    South,
    West,
    East,
}

type Coordinate = (usize, usize);
type Position = (Coordinate, Dir);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Position,
    steps: u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct StateKey {
    position: Position,
    dir: Dir,
    steps: u8,
}

impl From<State> for StateKey {
    fn from(value: State) -> Self {
        Self {
            position: value.position,
            dir: value.position.1,
            steps: value.steps,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.0.cmp(&other.position.0))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn heat_loss_ultra(input: Vec<&str>) -> usize {
    dijkstra(&input, 4, 10)
}

pub fn heat_loss(input: Vec<&str>) -> usize {
    dijkstra(&input, 1, 3)
}

fn dijkstra(input: &Vec<&str>, min_steps: u8, max_steps: u8) -> usize {
    let target = ((input.len() - 1), (input[0].len() - 1));

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

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(east_start);
    priority_queue.push(south_start);

    let mut distances: HashMap<StateKey, usize> = HashMap::new();
    distances.insert(east_start.into(), 0);
    distances.insert(south_start.into(), 0);

    while let Some(
        state @ State {
            cost,
            position,
            steps,
        },
    ) = priority_queue.pop()
    {
        if position.0 == target && steps >= min_steps {
            return cost;
        }

        if distances.contains_key(&state.into()) && distances.get(&state.into()).unwrap() < &cost {
            continue;
        }

        for next_position in next_positions(input, &position) {
            let heat = heat(input, &next_position.0);
            let new_heat = cost + heat as usize;

            let curr_dir = position.1;
            let next_dir = next_position.1;

            let next_state = State {
                cost: new_heat,
                position: next_position,
                steps: if curr_dir == next_dir { steps + 1 } else { 1 },
            };

            if next_state.steps > max_steps
                || distances.contains_key(&next_state.into())
                    && distances.get(&next_state.into()).unwrap() <= &new_heat
            {
                continue;
            }

            if curr_dir != next_dir && steps < min_steps {
                continue;
            }

            distances.insert(next_state.into(), new_heat);
            priority_queue.push(next_state);
        }
    }

    0
}

fn next_positions(input: &Vec<&str>, pos: &Position) -> Vec<Position> {
    let ((r, c), dir) = pos;

    vec![
        ((r.saturating_sub(1), *c), Dir::North),
        ((r + 1, *c), Dir::South),
        ((*r, c.saturating_sub(1)), Dir::West),
        ((*r, c + 1), Dir::East),
    ]
    .into_iter()
    .filter(|(x, d)| *d != opposite_dir(dir) && *x != pos.0 && in_bound(input, *x))
    .collect()
}

fn opposite_dir(dir: &Dir) -> Dir {
    match dir {
        Dir::North => Dir::South,
        Dir::South => Dir::North,
        Dir::West => Dir::East,
        Dir::East => Dir::West,
    }
}

fn heat(input: &Vec<&str>, (r, c): &Coordinate) -> u32 {
    let heat = input.get(*r).unwrap().chars().nth(*c).unwrap();

    heat.to_digit(10).unwrap()
}

fn in_bound(input: &Vec<&str>, coord: Coordinate) -> bool {
    let (r, c) = coord;

    r < input.len() && c < input[0].len()
}
