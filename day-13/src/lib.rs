pub fn notes(input: Vec<&str>) -> u32 {
    input
        .split(|x| x.is_empty())
        .map(|x| x.to_vec())
        .map(calc_reflection)
        .sum()
}

fn calc_reflection(input: Vec<&str>) -> u32 {
    let vert = reflection(transpose(&input));
    let hori = reflection(input.iter().map(|x| x.to_string()).collect());

    vert + hori * 100
}

fn reflection(input: Vec<String>) -> u32 {
    let hori_len = input.len();
    let mirror_pos = find_mirror(input.iter().map(|x| x.to_string()).collect());

    for m_pos in mirror_pos.iter() {
        let top = (0..*m_pos)
            .rev()
            .map(|x| input.get(x as usize).unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let bottom = (m_pos + 2..hori_len as u32)
            .map(|x| input.get(x as usize).unwrap().chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if top.len() <= bottom.len() {
            let is_mirror = top.iter().zip(bottom.iter()).all(|(l, r)| l == r);
            if is_mirror {
                return top.len() as u32 + 1;
            }
        } else {
            let is_mirror = bottom.iter().zip(top.iter()).all(|(l, r)| l == r);
            if is_mirror {
                return top.len() as u32 + 1;
            }
        }
    }

    0
}

fn find_mirror(input: Vec<String>) -> Vec<u32> {
    input
        .iter()
        .zip(input.iter().skip(1))
        .enumerate()
        .filter(|(_, (l, r))| l == r)
        .map(|(i, _)| i as u32)
        .collect()
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
fn should_count_columns_for_a_vertical_mirror() {
    let input = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ];

    let count = calc_reflection(input);

    assert_eq!(count, 5);
}
