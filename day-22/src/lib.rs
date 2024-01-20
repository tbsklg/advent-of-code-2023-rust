use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
};

pub fn bricks(collect: Vec<&str>) -> usize {
    let mut bricks = parse(collect);
    bricks.sort_by_key(|x| x.z.0);

    let mut fallen_bricks = fall(&bricks);
    fallen_bricks.sort_by_key(|x| x.z.0);

    disintegrate(&fallen_bricks)
}

pub fn all_bricks(collect: Vec<&str>) -> usize {
    let mut bricks = parse(collect);
    bricks.sort_by_key(|x| x.z.0);

    let mut fallen_bricks = fall(&bricks);
    fallen_bricks.sort_by_key(|x| x.z.0);

    disintegrate_chain(&fallen_bricks)
}

fn fall(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut fallen_bricks = vec![];

    for (i, brick) in bricks.iter().enumerate() {
        let mut max_z = 1;
        for fallen in &fallen_bricks[..i] {
            if brick.overlaps(fallen) {
                max_z = std::cmp::max(max_z, fallen.z.1 + 1);
            }
        }

        fallen_bricks.push(Brick {
            x: brick.x,
            y: brick.y,
            z: (max_z, brick.z.1 - (brick.z.0 - max_z)),
        });
    }

    fallen_bricks
}

fn disintegrate_chain(bricks: &Vec<Brick>) -> usize {
    let mut k_supports_v = HashMap::from(
        bricks
            .iter()
            .enumerate()
            .map(|(i, _)| (i, HashSet::new()))
            .collect::<HashMap<usize, HashSet<usize>>>(),
    );

    let mut v_supports_k = k_supports_v.clone();

    for (j, upper) in bricks.iter().enumerate() {
        for (i, lower) in bricks[..j].iter().enumerate() {
            if upper.lays_above(lower) {
                k_supports_v.get_mut(&i).unwrap().insert(j);
                v_supports_k.get_mut(&j).unwrap().insert(i);
            }
        }
    }

    let mut total = 0;
    for (i, _) in bricks.iter().enumerate() {
        let mut queue = VecDeque::from(
            k_supports_v
                .get(&i)
                .unwrap()
                .iter()
                .filter(|x| v_supports_k.get(x).unwrap().len() == 1)
                .collect::<Vec<&usize>>(),
        );

        let mut falling: HashSet<usize> = HashSet::from_iter(queue.iter().map(|x| **x));
        falling.insert(i);

        while !queue.is_empty() {
            let j = queue.pop_front().unwrap();

            for k in k_supports_v
                .get(j)
                .unwrap()
                .iter()
                .filter(|x| !falling.contains(x))
                .collect::<Vec<&usize>>()
            {
                if v_supports_k.get(k).unwrap().is_subset(&falling) {
                    queue.push_back(k);
                    falling.insert(*k);
                }
            }
        }

        total += falling.len() - 1;
    }

    total
}

fn disintegrate(bricks: &Vec<Brick>) -> usize {
    let mut disintegrated = HashSet::new();

    for (i, brick) in bricks.iter().enumerate() {
        let further_bricks = &bricks[i + 1..];

        let bricks_above = further_bricks
            .iter()
            .filter(|x| x.lays_above(brick))
            .collect::<Vec<&Brick>>();

        let all_above_connected = bricks_above.iter().all(|x| {
            let bricks_below = &bricks.iter().filter(|y| y.lays_below(x)).count();
            bricks_below >= &2
        });

        if all_above_connected {
            disintegrated.insert(*brick);
        }
    }

    disintegrated.len()
}

type Range = (usize, usize);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Brick {
    x: Range,
    y: Range,
    z: Range,
}

impl Brick {
    fn from(input: &str) -> Self {
        let mut slice = input.split('~');
        let from = parse_pos(slice.next().unwrap());
        let to = parse_pos(slice.next().unwrap());

        let mut ranges = from.iter().zip(to.iter()).map(|(x, y)| (*x, *y));

        Self {
            x: ranges.next().unwrap(),
            y: ranges.next().unwrap(),
            z: ranges.next().unwrap(),
        }
    }

    fn lays_above(&self, other: &Self) -> bool {
        self.z.0 == other.z.1 + 1 && self.overlaps(other)
    }

    fn lays_below(&self, other: &Self) -> bool {
        self.z.1 == other.z.0 - 1 && self.overlaps(other)
    }

    fn overlaps(&self, other: &Self) -> bool {
        max(self.x.0, other.x.0) <= min(self.x.1, other.x.1)
            && max(self.y.0, other.y.0) <= min(self.y.1, other.y.1)
    }
}

fn parse_pos(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn parse(input: Vec<&str>) -> Vec<Brick> {
    input.iter().map(|x| Brick::from(x)).collect()
}

#[test]
fn should_calculate_bricks_to_be_disintegrated() {
    let input = vec![
        "1,0,1~1,2,1",
        "0,0,2~2,0,2",
        "0,2,3~2,2,3",
        "0,0,4~0,2,4",
        "2,0,5~2,2,5",
        "0,1,6~2,1,6",
        "1,1,8~1,1,9",
    ];

    let bricks = bricks(input);

    assert_eq!(bricks, 5);
}
