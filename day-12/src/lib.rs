pub fn arrangements(input: Vec<&str>) -> u32 {
    input
        .iter()
        .map(|x| x.split(' '))
        .map(|mut x| {
            (
                x.next().unwrap().clone().to_string(),
                x.next()
                    .unwrap()
                    .clone()
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap().clone())
                    .collect::<Vec<u32>>(),
            )
        })
        .map(|(input, groups)| count_arrangements(input, groups).unwrap_or(0))
        .sum()
}

fn count_arrangements(input: String, groups: Vec<u32>) -> Option<u32> {
    fn arrangements(input: String, groups: Vec<u32>, count: u32) -> Option<u32> {
        let has_question_mark = input.contains('?');
        let is_valid = is_valid(input.clone(), groups.clone());

        match (has_question_mark, is_valid) {
            (true, false) => Some(
                arrangements(input.replacen("?", "#", 1), groups.clone(), count).unwrap_or(0)
                    + arrangements(input.replacen("?", ".", 1), groups.clone(), count).unwrap_or(0),
            ),
            (false, true) => Some(count + 1),
            _ => None,
        }
    }

    arrangements(input.clone(), groups.clone(), 0)
}

fn is_valid(input: String, groups: Vec<u32>) -> bool {
    !input.contains('?')
        && input
            .split('.')
            .filter(|x| !x.is_empty() && !x.contains('?'))
            .map(|x| x.len())
            .zip(groups)
            .all(|(x, y)| x == y as usize)
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
