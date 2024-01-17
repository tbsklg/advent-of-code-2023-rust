pub fn bricks(collect: Vec<&str>) -> usize {
    let bricks = parse(collect);
    let fallen_bricks = fall(&bricks);

    disintegrate(&fallen_bricks)
}

fn fall(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut fallen_bricks = vec![];

    for brick in bricks {
        if fallen_bricks.is_empty() {
            fallen_bricks.push(brick.clone());
            continue;
        }
    }

    fallen_bricks
}

fn disintegrate(bricks: &Vec<Brick>) -> usize {
    let mut disintegrated = vec![];

    for brick in bricks {
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
        (self.x.0 <= other.x.1 && other.x.0 <= self.x.1)
            || (self.y.0 <= other.y.1 && other.y.0 <= self.y.1)
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
