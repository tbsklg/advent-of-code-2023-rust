pub fn tilt_cycle(input: Vec<&str>, times: usize) -> u64 {
    let input = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();

    let mut seen = vec![];
    let mut final_index = 0;

    let mut result = input.clone();

    for i in 0..times {
        result = cycle(result);
        if seen.contains(&result) {
            final_index =
                (times - (times - i)) % (i - seen.iter().position(|x| x == &result).unwrap());
            break;
        }
        seen.push(result.clone());
    }

    load(seen[final_index].clone())
}

pub fn count_north(input: Vec<&str>) -> u64 {
    let input = input.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    let tilted_north = tilt_north(input);

    load(tilted_north)
}

fn load(input: Vec<String>) -> u64 {
    let row_numbers = (1..input.len() + 1).rev().collect::<Vec<usize>>();

    row_numbers
        .iter()
        .zip(input.iter())
        .map(|(line_number, line)| {
            let os = line.chars().filter(|&x| x == 'O').count();
            line_number * os
        })
        .sum::<usize>() as u64
}

fn cycle(input: Vec<String>) -> Vec<String> {
    let mut result = input.clone();

    for _ in 0..4 {
        result = rotate_90(tilt_north(result));
    }

    result
}

fn tilt_north(input: Vec<String>) -> Vec<String> {
    let rotated = transpose(input.iter().map(|x| x.to_string()).collect::<Vec<String>>());

    let rolled = rotated
        .iter()
        .map(|x| roll_left(x.to_string()))
        .collect::<Vec<String>>();

    return transpose(rolled);
}

fn roll_left(line: String) -> String {
    let split_by_rock = line.split('#').collect::<Vec<&str>>();

    let move_os_left = split_by_rock
        .iter()
        .map(|x| {
            let os = x.chars().filter(|&x| x == 'O').collect::<String>();
            let dots = x.chars().filter(|&x| x == '.').collect::<String>();

            os + &dots
        })
        .collect::<Vec<String>>()
        .join("#");

    move_os_left
}

fn rotate_90(input: Vec<String>) -> Vec<String> {
    let mut result = vec![];

    for i in 0..input[0].len() {
        let line = input
            .iter()
            .map(|x| x.chars().nth(i).unwrap())
            .rev()
            .collect::<String>();
        result.push(line);
    }

    result
}

fn transpose(input: Vec<String>) -> Vec<String> {
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
fn should_roll_left() {
    let input = "OO.#O....O".to_string();

    let rolled = roll_left(input);

    assert_eq!(rolled, "OO.#OO....".to_string());
}

#[test]
fn should_tilt_towards_north() {
    let input = vec![
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    let result = count_north(input);

    assert_eq!(result, 136);
}
