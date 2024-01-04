#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn from(value: &str) -> Self {
        match value {
            "U" => Dir::North,
            "D" => Dir::South,
            "L" => Dir::West,
            "R" => Dir::East,
            _ => panic!("No dir for input"),
        }
    }

    fn from_rgb(value: char) -> Dir {
        match value {
            '0' => Dir::East,
            '1' => Dir::South,
            '2' => Dir::West,
            '3' => Dir::North,
            _ => panic!("No dir for input"),
        }
    }
}

#[derive(Debug)]
struct Dig {
    dir: Dir,
    steps: i64,
}

impl Dig {
    fn from(value: &str) -> Self {
        let mut slice = value.split(' ');

        Self {
            dir: Dir::from(slice.next().unwrap()),
            steps: slice.next().unwrap().parse::<i64>().unwrap(),
        }
    }

    fn from_rgb(value: &str) -> Self {
        let mut slice = value.split(' ');
        let rgb = slice.nth(2).unwrap().replace(['(', ')'], "");

        Self {
            dir: Dir::from_rgb(rgb.chars().last().unwrap()),
            steps: to_base_10(&rgb[1..rgb.len() - 1]),
        }
    }
}

pub fn cubic_meters(input: Vec<&str>) -> i64 {
    let digs = parse(input);
    let path = path(digs).into_iter().skip(1).collect();
    let area = shoelace_formula(&path);

    path.len() as i64 + area + 1 - (path.len() / 2) as i64
}

pub fn cubic_meters_rgb(input: Vec<&str>) -> i64 {
    let digs = parse_rgb(input);

    let path = path(digs).into_iter().skip(1).collect();
    let area = shoelace_formula(&path);

    path.len() as i64 + area + 1 - (path.len() / 2) as i64
}

fn to_base_10(hex: &str) -> i64 {
    i64::from_str_radix(hex, 16).unwrap()
}

fn path(digs: Vec<Dig>) -> Vec<(i64, i64)> {
    digs.iter().fold(vec![(0, 0)], |acc, dig| {
        let last = acc.last().unwrap();
        [acc.clone(), points(last, dig)].concat()
    })
}

fn points(start: &(i64, i64), dig: &Dig) -> Vec<(i64, i64)> {
    let (r, c) = start;

    match dig.dir {
        Dir::North => ((r - dig.steps)..*r).rev().map(|x| (x, *c)).collect(),
        Dir::South => (*r + 1..*r + 1 + dig.steps).map(|x| (x, *c)).collect(),
        Dir::West => ((c - dig.steps)..*c).rev().map(|x| (*r, x)).collect(),
        Dir::East => (*c + 1..*c + 1 + dig.steps).map(|x| (*r, x)).collect(),
    }
}

fn parse(input: Vec<&str>) -> Vec<Dig> {
    input.iter().map(|x| Dig::from(x)).collect()
}

fn parse_rgb(input: Vec<&str>) -> Vec<Dig> {
    input.iter().map(|x| Dig::from_rgb(x)).collect()
}

fn shoelace_formula(points: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += points[i].0 * points[j].1 - points[j].0 * points[i].1;
    }

    sum.abs() / 2
}
