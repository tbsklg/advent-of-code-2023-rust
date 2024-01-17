use std::collections::VecDeque;

pub fn bricks(collect: Vec<&str>) -> usize {
    let bricks = parse(collect);

    disintegrate(&bricks)
}

fn disintegrate(bricks: &Vec<Brick>) -> usize {
    let mut disintegrated = vec![];

    for brick in bricks {
        let connected = connected_up(&brick, &bricks);

        if connected.len() == 0 {
            disintegrated.push(brick);
        }

        assert!(connected.len() == 1);

        let connected = connected_down(&brick, &bricks);

        if connected.len() > 1 {
            disintegrated.push(brick);
        }
    }

    println!("Disintegrated: {:?}", disintegrated);

    disintegrated.len()
}

fn connected_down(brick: &Brick, bricks: &Vec<Brick>) -> Vec<&Brick> {
    let connected = bricks
        .iter()
        .filter(|b| {
            lays_down(&b.z, &brick.z) && (overlap(&b.x, &brick.x) || overlap(&b.y, &brick.y))
        })
        .collect::<Vec<&Brick>>();
}

fn connected_up(brick: &Brick, bricks: &Vec<Brick>) -> Vec<&Brick> {
    let connected = bricks
        .iter()
        .filter(|b| {
            lays_above(&b.z, &brick.z) && (overlap(&b.x, &brick.x) || overlap(&b.y, &brick.y))
        })
        .collect::<Vec<&Brick>>();
}

fn lays_down(z_1: &(usize, usize), z_2: &(usize, usize)) -> bool {
    z_1.0 + 1 == z_2.1
}

fn lays_above(z_1: &(usize, usize), z_2: &(usize, usize)) -> bool {
    z_1.1 + 1 == z_2.0
}

fn overlap((x1, y1): &(usize, usize), (x2, y2): &(usize, usize)) -> bool {
    x1 <= y2 && y1 <= x2
}

type Range = (usize, usize);

#[derive(Debug)]
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
