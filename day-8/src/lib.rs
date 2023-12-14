use lazy_static::lazy_static;
use num_integer::lcm;
use regex::Regex;
use std::{collections::HashMap, iter::Cycle, str::Chars};

fn find_starting_nodes(network: HashMap<String, (String, String)>) -> Vec<String> {
    network
        .iter()
        .filter(|(n, (_, _))| n.ends_with('A'))
        .map(|(n, (_, _))| n.clone())
        .collect::<Vec<String>>()
}

lazy_static! {
    static ref WORDS_REGEX: Regex = Regex::new(r"\w+").unwrap();
}

pub fn calculate_steps(input: Vec<&str>, start_node: &str, end_node: &str) -> u64 {
    let mut directions = parse_directions(input.first().unwrap());
    let network = create_network(input.get(2..).unwrap().to_vec());

    let mut steps = 0;
    let mut current_node = start_node;

    while !current_node.ends_with(end_node) {
        let (left, right) = network.get(current_node).unwrap();
        let dir = directions.next().unwrap();

        current_node = match dir {
            'L' => left,
            'R' => right,
            _ => panic!("Unknown direction"),
        };

        steps += 1
    }

    steps
}

pub fn calcualte_steps_with_multiple_starts(input: Vec<&str>) -> u64 {
    let network = create_network(input.get(2..).unwrap().to_vec());

    let starting_nodes = find_starting_nodes(network);

    let steps = starting_nodes
        .iter()
        .map(|x| calculate_steps(input.clone(), x, "Z"))
        .collect::<Vec<u64>>();

    calculate_lcm(steps)
}

fn calculate_lcm(steps: Vec<u64>) -> u64 {
    steps.iter().fold(steps[0], |acc, x| lcm(acc, *x))
}

fn create_network(input: Vec<&str>) -> HashMap<String, (String, String)> {
    input
        .iter()
        .map(|x| parse_network_line(x))
        .collect::<HashMap<String, (String, String)>>()
}

fn parse_network_line(input: &str) -> (String, (String, String)) {
    let words = WORDS_REGEX
        .find_iter(input)
        .map(|x| x.as_str().to_string())
        .collect::<Vec<String>>();

    (
        words.get(0).unwrap().clone(),
        (words.get(1).unwrap().clone(), words.get(2).unwrap().clone()),
    )
}

fn parse_directions(input: &str) -> Cycle<Chars> {
    input.chars().cycle()
}

#[test]
fn shoud_parse_directions_as_cycle() {
    let input = "LLR";

    let mut directions = parse_directions(input);

    assert_eq!(directions.next(), Some('L'));
    assert_eq!(directions.next(), Some('L'));
    assert_eq!(directions.next(), Some('R'));
    assert_eq!(directions.next(), Some('L'));
}

#[test]
fn should_parse_network_line() {
    let input = "AAA = (BBB, CCC)";

    let node = parse_network_line(input);

    assert_eq!(
        node,
        ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()))
    );
}

#[test]
fn should_create_network() {
    let input = vec!["AAA = (BBB, CCC)", "CCC = (DDD, EEE)", "DDD = (EEE, FFF)"];

    let network = create_network(input);

    assert_eq!(network.len(), 3);
    assert_eq!(
        network.get(&"AAA".to_string()),
        Some(&("BBB".to_string(), "CCC".to_string()))
    );
}

#[test]
fn should_find_starting_nodes_that_end_with_a() {
    let input = vec!["22A = (22B, XXX)", "11A = (11B, XXX)", "33C = (33B, XXX)"];

    let network = create_network(input);
    let starting_nodes = find_starting_nodes(network);

    assert_eq!(starting_nodes, vec!["11A", "22A"]);
}
