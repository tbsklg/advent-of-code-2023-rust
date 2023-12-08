use std::iter::zip;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

pub fn records(input: Vec<&str>) -> u64 {
    parse_document(input)
        .iter()
        .map(|x| number_of_records(*x) as u64)
        .product()
}

fn number_of_records(input: (u64, u64)) -> usize {
    let (time, dist) = input;

    (1..time).filter(|i| (time - i) * i > dist).count()
}

fn parse_document(input: Vec<&str>) -> Vec<(u64, u64)> {
    let time = parse_line(input.first().unwrap());
    let dist = parse_line(input.get(1).unwrap());

    zip(time, dist).collect()
}

fn parse_line(input: &str) -> Vec<u64> {
    NUMBER_REGEX
        .find_iter(input)
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect()
}

#[test]
fn should_parse_document() {
    let input = vec!["Time:      7  15   30", "Distance:  9  40  200"];

    let doc = parse_document(input);

    assert_eq!(doc, vec![(7, 9), (15, 40), (30, 200)]);
}

#[test]
fn should_calculate_the_number_of_ways_to_beat_the_record() {
    let input = (7, 9);

    let number_of_records = number_of_records(input);

    assert_eq!(number_of_records, 4);
}
