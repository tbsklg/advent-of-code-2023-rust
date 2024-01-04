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
}

struct Dig {
    dir: Dir,
    steps: i32,
    rgb: String,
}

impl Dig {
    fn from(value: &str) -> Self {
        let mut slice = value.split(' ');

        Self {
            dir: Dir::from(slice.next().unwrap()),
            steps: slice.next().unwrap().parse::<i32>().unwrap(),
            rgb: slice.next().unwrap().to_string(),
        }
    }
}

pub fn cubic_meters(input: Vec<&str>) -> i32 {
    let digs = parse(input);
    let path = path(digs).into_iter().skip(1).collect();
    let area = shoelace_formula(&path);

    path.len() as i32 + area + 1 - (path.len() / 2) as i32
}

fn path(digs: Vec<Dig>) -> Vec<(i32, i32)> {
    digs.iter().fold(vec![(0, 0)], |acc, dig| {
        let last = acc.last().unwrap();
        [acc.clone(), points(last, dig)].concat()
    })
}

fn points(start: &(i32, i32), dig: &Dig) -> Vec<(i32, i32)> {
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

fn shoelace_formula(points: &Vec<(i32, i32)>) -> i32 {
    let mut sum = 0;

    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        sum += points[i].0 * points[j].1 - points[j].0 * points[i].1;
    }

    sum.abs() / 2
}
