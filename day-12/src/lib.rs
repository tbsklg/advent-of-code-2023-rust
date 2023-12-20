pub fn arrangements(input: Vec<&str>) -> u32 {
    input
        .iter()
        .map(|x| x.split(' '))
        .map(|mut x| {
            (
                x.next().unwrap().to_string(),
                x.next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(input, groups)| count_arrangements(input, groups).unwrap_or(0))
        .sum()
}

fn count_arrangements(input: String, groups: Vec<u32>) -> Option<u32> {
    fn arrangements(input: String, groups: Vec<u32>, count: u32) -> Option<u32> {
        if input.contains('?') {
            Some(
                arrangements(input.replacen('?', "#", 1), groups.clone(), count).unwrap_or(0)
                    + arrangements(input.replacen('?', ".", 1), groups, count).unwrap_or(0),
            )
        } else if is_valid(input, groups) {
            Some(count + 1)
        } else {
            None
        }
    }

    arrangements(input, groups, 0)
}

fn is_valid(input: String, groups: Vec<u32>) -> bool {
    let split_by_dot = input
        .split('.')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    split_by_dot.len() == groups.len()
        && split_by_dot
            .iter()
            .zip(groups)
            .all(|(x, y)| x.len() == y as usize)
}

#[test]
fn should_validate_input_with_question_marks() {
    let input = "???.###";
    let groups = vec![1, 1, 3];

    let arrangements = is_valid(input.to_string(), groups);

    assert_eq!(arrangements, false);
}

#[test]
fn should_validate_input_without_question_marks() {
    let valid_inputs = vec![
        ("#.#.###", vec![1, 1, 3]),
        (".#...#....###.", vec![1, 1, 3]),
        (".#.###.#.######", vec![1, 3, 1, 6]),
        ("####.#...#...", vec![4, 1, 1]),
    ];

    for (input, groups) in valid_inputs {
        let arrangements = is_valid(input.to_string(), groups);

        assert_eq!(arrangements, true);
    }
}

#[test]
fn should_count_arrangements() {
    let input = ".??..??...?##.";
    let groups = vec![1, 1, 3];

    let arrangements = count_arrangements(input.to_string(), groups);

    assert_eq!(arrangements, Some(4));
}

#[test]
fn should_count_arrangements_for_large_example() {
    let input = "?###????????";
    let groups = vec![3, 2, 1];

    let arrangements = count_arrangements(input.to_string(), groups);

    assert_eq!(arrangements, Some(10));
}
