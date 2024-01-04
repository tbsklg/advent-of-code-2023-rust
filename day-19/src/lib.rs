use std::collections::HashMap;

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
                slice.next().unwrap().chars().nth(0).unwrap(),
                slice.next().unwrap().parse::<usize>().unwrap(),
                Box::new(Rule::from(slice.next().unwrap())),
            );
        }

        if input.contains('>') {
            return Self::GT(
                slice.next().unwrap().chars().nth(0).unwrap(),
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

fn calc_parts(ratings: &Vec<Vec<(char, usize)>>, workflows: &HashMap<&str, Vec<Rule>>) -> usize {
    ratings
        .iter()
        .filter(|x| calc_part(x, workflows) == Rule::Accepted)
        .map(|x| x.iter().map(|(_, i)| i).sum::<usize>())
        .sum()
}

fn calc_part(rating: &Vec<(char, usize)>, workflows: &HashMap<&str, Vec<Rule>>) -> Rule {
    match calc_rating(rating, workflows, "in") {
        Rule::Next(s) => calc_rating(rating, workflows, &s),
        Rule::Accepted => Rule::Accepted,
        _ => Rule::Rejected,
    }
}

fn calc_rating(
    rating: &Vec<(char, usize)>,
    workflows: &HashMap<&str, Vec<Rule>>,
    step: &str,
) -> Rule {
    println!("Step: {}", step);
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
            Rule::Next(s) => curr_rule = calc_rating(rating, workflows, s),
            Rule::Accepted => curr_rule = Rule::Accepted,
            Rule::Rejected => curr_rule = Rule::Rejected,
        };
    }

    curr_rule
}

fn parse_ratings(input: Vec<&str>) -> Vec<Vec<(char, usize)>> {
    input.iter().map(|x| parse_rating(x)).collect()
}

fn parse_rating(input: &str) -> Vec<(char, usize)> {
    input
        .replace("{", "")
        .replace("}", "")
        .split(',')
        .map(|x| {
            let mut slice = x.split('=');
            (
                slice.next().unwrap().chars().nth(0).unwrap(),
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
            .replace("}", "")
            .split(',')
            .map(|x| Rule::from(x))
            .collect();

        acc.insert(name, rules);
        acc
    })
}
