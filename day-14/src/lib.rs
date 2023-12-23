pub fn tilt_north(input: Vec<&str>) -> u64 {
    let rotated = transpose(input.iter().map(|x| x.to_string()).collect::<Vec<String>>());

    let rolled = rotated
        .iter()
        .map(|x| roll_left(x.to_string()))
        .collect::<Vec<String>>();

    let rotated_back = transpose(rolled);

    let row_numbers = (1..rotated_back.len() + 1).rev().collect::<Vec<usize>>();

    row_numbers
        .iter()
        .zip(rotated_back.iter())
        .map(|(line_number, line)| {
            let os = line.chars().filter(|&x| x == 'O').count();
            line_number * os
        })
        .sum::<usize>() as u64
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

    let result = tilt_north(input);

    assert_eq!(result, 136);
}
