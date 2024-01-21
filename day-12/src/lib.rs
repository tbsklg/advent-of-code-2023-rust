use std::{collections::HashMap, iter::repeat};

pub fn arrangements(input: Vec<&str>) -> u64 {
    let cache = &mut HashMap::<(String, Vec<u32>), u64>::new();

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
        .map(|(input, groups)| count_arrangements(input, groups, cache))
        .sum()
}

pub fn arrangements_five(input: Vec<&str>) -> u64 {
    let cache = &mut HashMap::<(String, Vec<u32>), u64>::new();

    input
        .iter()
        .map(|x| x.split(' '))
        .map(|mut x| {
            (
                repeat(x.next().unwrap().to_string())
                    .take(5)
                    .collect::<Vec<String>>()
                    .join("?"),
                x.next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
                    .repeat(5),
            )
        })
        .map(|(input, groups)| count_arrangements(input, groups, cache))
        .sum()
}

fn count_arrangements(
    input: String,
    groups: Vec<u32>,
    cache: &mut HashMap<(String, Vec<u32>), u64>,
) -> u64 {
    if input.is_empty() {
        return match groups.is_empty() {
            true => 1,
            false => 0,
        };
    }

    if groups.is_empty() {
        return match input.contains('#') {
            true => 0,
            false => 1,
        };
    }

    let head_group = *groups.iter().nth(0).unwrap();
    let head_input = input.chars().next().unwrap();
    let input_len = input.len() as u32;

    if cache.contains_key(&(input.clone(), groups.clone())) {
        return *cache.get(&(input, groups)).unwrap();
    }

    let mut result = 0_u64;

    if head_input == '.' || head_input == '?' {
        result += count_arrangements(input[1..].to_string(), groups.clone(), cache);
    }

    if (head_input == '#' || head_input == '?')
        && head_group <= input_len
        && !input.chars().take(head_group as usize).any(|x| x == '.')
        && (head_group == input_len || input.chars().nth(head_group as usize).unwrap() != '#')
    {
        result += count_arrangements(
            input.chars().skip(head_group as usize + 1).collect(),
            groups.clone().into_iter().skip(1).collect(),
            cache,
        );
    }

    cache.insert((input, groups), result);

    result
}

#[test]
fn should_count_arrangements() {
    let input = ".??..??...?##.";
    let groups = vec![1, 1, 3];

    let arrangements = count_arrangements(input.to_string(), groups, &mut HashMap::new());

    assert_eq!(arrangements, 4);
}

#[test]
fn should_count_arrangements_for_large_example() {
    let input = "?###????????";
    let groups = vec![3, 2, 1];

    let arrangements = count_arrangements(input.to_string(), groups, &mut HashMap::new());

    assert_eq!(arrangements, 10);
}
