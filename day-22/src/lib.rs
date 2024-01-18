use std::cmp::{max, min};

pub fn bricks(collect: Vec<&str>) -> usize {
    let mut bricks = parse(collect);
    bricks.sort_by_key(|x| x.z.0);

    let mut fallen_bricks = fall(&bricks);
    fallen_bricks.sort_by_key(|x| x.z.0);

    disintegrate(&fallen_bricks)
}

fn fall(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut fallen_bricks = vec![];

    for (i, brick) in bricks.iter().enumerate() {
        let mut max_z = 1;
        for fallen in &bricks[0..i] {
            if brick.overlaps(&fallen) {
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

fn disintegrate(bricks: &Vec<Brick>) -> usize {
    let mut disintegrated = vec![];

    for (i, brick) in bricks.iter().enumerate() {
        let no_brick_above = bricks.iter().all(|x| !x.lays_above(&brick));
        if no_brick_above {
            disintegrated.push(brick);
            continue;
        }

        let bricks_above = bricks
            .iter()
            .filter(|x| x.lays_above(&brick))
            .collect::<Vec<&Brick>>();

        let all_above_connected = bricks_above.iter().all(|x| {
            let bricks_below = bricks
                .iter()
                .filter(|y| y.lays_below(&x))
                .collect::<Vec<&Brick>>();

            bricks_below.len() >= 2
        });

        if all_above_connected {
            disintegrated.push(brick);
        }
    }

    disintegrated.len()
}

type Range = (usize, usize);

#[derive(Debug, Copy, Clone)]
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
        self.z.0 == other.z.1 + 1 && self.overlaps(&other)
    }

    fn lays_below(&self, other: &Self) -> bool {
        self.z.1 + 1 == other.z.0 && self.overlaps(&other)
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
    input.iter().map(|x| Brick::from(*x)).collect()
}
