use std::collections::HashSet;

pub fn plots(collect: Vec<&str>, steps: usize) -> usize {
    let mut curr_pos = HashSet::from([start(collect.clone())]);

    for _ in 0..steps {
        curr_pos = next_poss(collect.clone(), curr_pos)
            .iter()
            .filter(|(r, c)| collect[*r].chars().nth(*c).unwrap() == '.')
            .copied()
            .collect();
    }

    curr_pos.len() + 1
}

type Pos = (usize, usize);

const START: char = 'S';

fn start(garden: Vec<&str>) -> Pos {
    garden
        .iter()
        .enumerate()
        .find(|(_, l)| l.contains(START))
        .map(|(r, l)| (r, l.find(START).unwrap()))
        .unwrap()
}

fn next_poss(garden: Vec<&str>, poss: HashSet<Pos>) -> HashSet<Pos> {
    poss.iter()
        .flat_map(|p| next_pos(garden.clone(), *p))
        .collect::<HashSet<_>>()
}

fn next_pos(garden: Vec<&str>, (r, c): Pos) -> Vec<Pos> {
    vec![
        (r + 1, c),
        (r, c + 1),
        (r.saturating_sub(1), c),
        (r, c.saturating_sub(1)),
    ]
    .iter()
    .filter(|p| in_garden(&garden, **p))
    .copied()
 
    
    .collect()
}

fn in_garden(garden: &Vec<&str>, pos: Pos) -> bool {
    let (r, c) = pos;
    r < garden.len() && c < garden[r].len()
}
