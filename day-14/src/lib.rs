pub fn tilt_north(input: Vec<&str>) -> u64 {
    let rotated = transpose(input);

    println!("{:?}", rotated);

    0
}

fn roll_left(line: String) -> String {
    let rocks = line
        .chars()
        .enumerate()
        .filter(|(_, x)| *x == '#')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    let dots = line
        .chars()
        .enumerate()
        .filter(|(_, x)| *x == '.')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    line.chars().enumerate().fold(line.clone(), |acc, (i, x)| {
        if x == '#' || x == '.' {
            return acc;
        }

        if x == 'O' {
            let dots_left = dots.iter().filter(|x| **x < i).collect::<Vec<&usize>>();
            let rocks_left = rocks.iter().filter(|x| **x < i).collect::<Vec<&usize>>();

            if dots_left.is_empty() {
                return acc;
            }

            let min_dot = dots_left.iter().min().unwrap();

            if rocks_left.is_empty() {
                return swap(acc, i, **min_dot);
            } else {
                let min_rock = rocks_left.iter().min().unwrap();

                if min_rock < min_dot {
                    return swap(acc, i, **min_dot);
                }
            }

            return acc;
        }

        acc
    })
}

fn swap(input: String, i: usize, j: usize) -> String {
    let mut chars = input.chars().collect::<Vec<char>>();
    chars.swap(i, j);
    chars.into_iter().collect::<String>()
}

fn transpose(input: Vec<&str>) -> Vec<String> {
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
fn should_swap() {
    let input = "O.OO#....#".to_string();

    let result = swap(input, 0, 1);

    assert_eq!(result, ".OOO#....#".to_string());
}

#[test]
fn should_roll_left() {
    let input = "OO.#O....O".to_string();

    let rolled = roll_left(input);

    assert_eq!(rolled, "OO.#0O....".to_string());
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
