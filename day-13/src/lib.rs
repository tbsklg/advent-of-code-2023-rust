pub fn notes(input: Vec<&str>, smudge: u32) -> u32 {
    input
        .split(|x| x.is_empty())
        .map(|x| x.to_vec())
        .map(|x| calc_reflection(x, smudge))
        .sum()
}

fn calc_reflection(input: Vec<&str>, smudge: u32) -> u32 {
    let vert = reflection(transpose(&input), smudge);
    let hori = reflection(input.iter().map(|x| x.to_string()).collect(), smudge);

    if hori > 0 {
        return hori * 100;
    }

    vert + hori * 100
}

fn reflection(input: Vec<String>, smudge: u32) -> u32 {
    let mirror_pos = find_mirror(input.iter().map(|x| x.to_string()).collect(), smudge);

    for m_pos in mirror_pos.iter() {
        let top = (0..*m_pos + 1)
            .rev()
            .map(|x| input.get(x as usize).unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let bottom = (m_pos + 1..input.len() as u32)
            .map(|x| input.get(x as usize).unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let is_mirror = top
            .iter()
            .zip(bottom.iter())
            .filter(|(l, r)| l != r)
            .count()
            == smudge as usize;

        if is_mirror {
            return top.len() as u32;
        }
    }

    0
}

fn find_mirror(input: Vec<String>, smudge: u32) -> Vec<u32> {
    input
        .iter()
        .zip(input.iter().skip(1))
        .enumerate()
        .filter(|(_, (l, r))| difference(l, r) <= smudge)
        .map(|(i, _)| i as u32)
        .collect()
}

fn difference(l: &str, r: &str) -> u32 {
    l.chars().zip(r.chars()).filter(|(l, r)| l != r).count() as u32
}

fn transpose(input: &Vec<&str>) -> Vec<String> {
    let mut result = vec![];

    for i in 0..input[0].len() {
        let line = input
            .iter()
            .map(|x| x.chars().nth(i).unwrap())
            .collect::<String>();
        result.push(line);
    }

    result
}

#[test]
fn should_count_notes_with_smudge_of_0() {
    let input = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ];

    let count = calc_reflection(input, 0);

    assert_eq!(count, 5);
}

#[test]
fn should_count_notes_with_smudge_of_1() {
    let input = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ];

    let count = calc_reflection(input, 1);

    assert_eq!(count, 300);
}
