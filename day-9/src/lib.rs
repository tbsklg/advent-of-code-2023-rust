use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"-?\d+").unwrap();
}

pub fn calc_extrapolated_values(input: Vec<&str>) -> u32 {
    input
        .iter()
        .map(|x| parse_line(x))
        .map(|x| {
            (
                x.last().unwrap().clone(),
                capture_till_difference_is_zero(x),
            )
        })
        .map(|(last, difference)| last + difference)
        .sum::<i32>() as u32
}

fn calc_differences(input: Vec<i32>) -> Vec<i32> {
    let tail = input.iter().skip(1).collect::<Vec<&i32>>();

    input
        .iter()
        .zip(tail)
        .map(|(x, y)| (y - x) as i32)
        .collect()
}

fn capture_till_difference_is_zero(input: Vec<i32>) -> i32 {
    let differences = calc_differences(input.clone()).clone();

    fn capture_till_zero(input: Vec<i32>, result: Vec<i32>) -> Vec<i32> {
        let all_zero = input.clone().iter().all(|x| *x == 0);

        match all_zero {
            true => result,
            false => {
                let last = input.last().unwrap().clone();
                let differences = calc_differences(input).clone();

                capture_till_zero(differences, [vec![last], result].concat())
            }
        }
    }

    capture_till_zero(differences, vec![]).iter().sum()
}

fn parse_line(input: &str) -> Vec<i32> {
    NUMBER_REGEX
        .find_iter(input)
        .map(|x| x.as_str().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

#[test]
fn should_parse_a_line() {
    let input = "0 3 6 9 12 15";

    let numbers = parse_line(input);

    assert_eq!(numbers, vec![0, 3, 6, 9, 12, 15]);
}

#[test]
fn should_parse_a_line_with_negative_numbers() {
    let input = "-4 -4 -4 -2 2 12 63 280";

    let numbers = parse_line(input);

    assert_eq!(numbers, vec![-4, -4, -4, -2, 2, 12, 63, 280]);
}

#[test]
fn should_calc_differences_for_negative_numbers() {
    let input = vec![-4, -4, -4, -2, 2, 12, 63, 280];

    let differences = calc_differences(input);

    assert_eq!(differences, vec![0, 0, 2, 4, 10, 51, 217]);
}

#[test]
fn should_calc_differences() {
    let input = vec![4, 2, 1, 2, 3];

    let differences = calc_differences(input);

    assert_eq!(differences, vec![-2, -1, 1, 1]);
}

#[test]
fn should_capture_differences_till_zero() {
    let input = vec![10, 13, 16, 21, 30, 45];

    let differences = capture_till_difference_is_zero(input);

    assert_eq!(differences, 23);
}
