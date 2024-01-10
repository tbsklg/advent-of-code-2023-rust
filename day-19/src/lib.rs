use std::{
    cmp::{max, min},
    collections::HashMap,
};

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Accepted,
    Rejected,
    Next(String),
    GT(char, usize, Box<Rule>),
    LT(char, usize, Box<Rule>),
}

impl Rule {
    fn from(input: &str) -> Self {
        let mut slice = input.split(['<', '>', ':']);

        if input.contains('<') {
            return Self::LT(
                slice.next().unwrap().chars().next().unwrap(),
                slice.next().unwrap().parse::<usize>().unwrap(),
                Box::new(Rule::from(slice.next().unwrap())),
            );
        }

        if input.contains('>') {
            return Self::GT(
                slice.next().unwrap().chars().next().unwrap(),
                slice.next().unwrap().parse::<usize>().unwrap(),
                Box::new(Rule::from(slice.next().unwrap())),
            );
        }

        if input == "A" {
            return Self::Accepted;
        }

        if input == "R" {
            return Self::Rejected;
        }

        Self::Next(input.to_string())
    }
}

pub fn rating(input: Vec<&str>) -> usize {
    let mut slice = input.split(|x| x == &"");
    let workflows = parse_workflows(slice.next().unwrap().to_vec());
    let ratings = parse_ratings(slice.next().unwrap().to_vec());

    calc_parts(&ratings, &workflows)
}

pub fn combinations(input: Vec<&str>) -> usize {
    let mut slice = input.split(|x| x == &"");
    let workflows = parse_workflows(slice.next().unwrap().to_vec());

    calc_combinations(&workflows)
}

fn calc_combinations(workflows: &HashMap<&str, Vec<Rule>>) -> usize {
    let ranges: &mut HashMap<char, (usize, usize)> = &mut HashMap::from([
        ('x', (1, 4000)),
        ('m', (1, 4000)),
        ('a', (1, 4000)),
        ('s', (1, 4000)),
    ]);

    calc_combination("in", ranges, workflows)
}

fn calc_combination(
    part: &str,
    ranges: &mut HashMap<char, (usize, usize)>,
    workflows: &HashMap<&str, Vec<Rule>>,
) -> usize {
    let rules = workflows.get(part).unwrap();

    let mut curr = 0;
    for rule in rules {
        match rule {
            Rule::LT(c, i, r) => {
                let (l, h) = *ranges.get(c).unwrap();
                ranges.insert(*c, (l, min(*i - 1, h)));

                let next = &mut ranges.clone();
                match *r.clone() {
                    Rule::Next(s) => curr += calc_combination(&s, next, workflows),

                    Rule::Accepted => {
                        curr += calc_result(next);
                    }
                    _ => curr += 0,
                }

                ranges.insert(*c, (max(*i, l), h));
            }
            Rule::GT(c, i, r) => {
                let (l, h) = *ranges.get(c).unwrap();
                ranges.insert(*c, (max(*i + 1, l), h));

                let next = &mut ranges.clone();
                match *r.clone() {
                    Rule::Next(s) => curr += calc_combination(&s, next, workflows),

                    Rule::Accepted => {
                        curr += calc_result(next);
                    }
                    _ => curr += 0,
                }

                ranges.insert(*c, (l, min(*i, h)));
            }
            Rule::Next(s) => {
                curr += calc_combination(s, ranges, workflows);
            }
            Rule::Accepted => {
                curr += calc_result(ranges);
            }
            Rule::Rejected => {
                curr += 0;
            }
        }
    }
    curr
}

fn calc_result(ranges: &HashMap<char, (usize, usize)>) -> usize {
    if ranges.iter().all(|(_, (l, h))| h >= l) {
        return ranges.iter().fold(1, |acc, (_, (l, h))| acc * (h - l + 1));
    }

    0
}

fn calc_parts(ratings: &Vec<Vec<(char, usize)>>, workflows: &HashMap<&str, Vec<Rule>>) -> usize {
    ratings
        .iter()
        .filter(|x| calc_part(x, workflows) == Rule::Accepted)
        .map(|x| x.iter().map(|(_, i)| i).sum::<usize>())
        .sum()
}

fn calc_part(rating: &Vec<(char, usize)>, workflows: &HashMap<&str, Vec<Rule>>) -> Rule {
    let mut start = calc_rating(rating, workflows, "in");

    while start != Rule::Accepted && start != Rule::Rejected {
        match start {
            Rule::Next(s) => start = calc_rating(rating, workflows, &s),
            _ => break,
        }
    }

    start
}

fn calc_rating(
    rating: &Vec<(char, usize)>,
    workflows: &HashMap<&str, Vec<Rule>>,
    step: &str,
) -> Rule {
    let rules = workflows.get(step).unwrap();

    let mut curr_rule = Rule::Rejected;

    for rule in rules {
        match rule {
            Rule::LT(c, i, r) => {
                if rating.iter().any(|x| x.0 == *c && x.1 < *i) {
                    curr_rule = *r.clone();
                    break;
                }
            }
            Rule::GT(c, i, r) => {
                if rating.iter().any(|x| x.0 == *c && x.1 > *i) {
                    curr_rule = *r.clone();
                    break;
                }
            }
            r => {
                curr_rule = r.clone();
                break;
            }
        };
    }

    curr_rule
}

fn parse_ratings(input: Vec<&str>) -> Vec<Vec<(char, usize)>> {
    input.iter().map(|x| parse_rating(x)).collect()
}

fn parse_rating(input: &str) -> Vec<(char, usize)> {
    input
        .replace(['{', '}'], "")
        .split(',')
        .map(|x| {
            let mut slice = x.split('=');
            (
                slice.next().unwrap().chars().next().unwrap(),
                slice.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn parse_workflows(input: Vec<&str>) -> HashMap<&str, Vec<Rule>> {
    input.iter().fold(HashMap::new(), |mut acc, curr| {
        let mut slices = curr.split('{');

        let name = slices.next().unwrap();
        let rules = slices
            .next()
            .unwrap()
            .replace('}', "")
            .split(',')
            .map(Rule::from)
            .collect();

        acc.insert(name, rules);
        acc
    })
}
