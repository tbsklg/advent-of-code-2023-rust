#![warn(clippy::pedantic)]

use lazy_static::lazy_static;
use regex::Regex;

struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Schematic {
    fn from_lines(lines: Vec<&str>) -> Self {
        let numbers = lines
            .iter()
            .enumerate()
            .flat_map(|(i, l)| extract_numbers_from_line(i, l))
            .collect();

        let symbols = lines
            .iter()
            .enumerate()
            .flat_map(|(i, l)| extract_symbols_from_line(i, l))
            .collect();

        Self { numbers, symbols }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Symbol {
    row_index: usize,
    col_index: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct Number {
    row_index: usize,
    col_from_index: usize,
    col_to_index: usize,
    value: u32,
}

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
    static ref SYMBOL_REGEX: Regex = Regex::new(r"[=#$+*%&@/-]").unwrap();
}

#[must_use]
pub fn sum_up_possible_numbers(lines: Vec<&str>) -> u32 {
    let schematic = Schematic::from_lines(lines);
    let possible_numbers = possible_numbers(schematic.numbers, schematic.symbols);

    possible_numbers.iter().map(|n| n.value).sum()
}

fn possible_numbers(numbers: Vec<Number>, symbols: Vec<Symbol>) -> Vec<Number> {
    numbers
        .into_iter()
        .filter(|n| has_symbol(n, symbols.clone()))
        .collect()
}

fn has_symbol(n: &Number, symbols: Vec<Symbol>) -> bool {
    symbols
        .into_iter()
        .any(|s| next_row(n, s) || current_row(n, s) || former_row(n, s))
}

fn next_row(n: &Number, s: Symbol) -> bool {
    n.row_index == s.row_index + 1
        && s.col_index >= n.col_from_index.saturating_sub(1)
        && s.col_index <= n.col_to_index + 1
}

fn current_row(n: &Number, s: Symbol) -> bool {
    n.row_index == s.row_index
        && s.col_index >= n.col_from_index.saturating_sub(1)
        && s.col_index <= n.col_to_index + 1
}

fn former_row(n: &Number, s: Symbol) -> bool {
    n.row_index == s.row_index - 1
        && s.col_index >= n.col_from_index.saturating_sub(1)
        && s.col_index <= n.col_to_index + 1
}

fn extract_symbols_from_line(line_number: usize, input: &str) -> Vec<Symbol> {
    SYMBOL_REGEX
        .find_iter(input)
        .map(|m| Symbol {
            row_index: line_number,
            col_index: m.start(),
        })
        .collect()
}

fn extract_numbers_from_line(line_number: usize, line: &str) -> Vec<Number> {
    NUMBER_REGEX
        .find_iter(line)
        .map(|m| Number {
            row_index: line_number,
            col_from_index: m.start(),
            col_to_index: m.end() - 1,
            value: m.as_str().parse::<u32>().unwrap(),
        })
        .collect()
}

#[test]
fn should_extract_numbers_from_a_line() {
    let input = "467......2";
    let line_number = 0;

    let numbers = extract_numbers_from_line(line_number, input);

    assert_eq!(numbers.len(), 2);
    assert_eq!(
        numbers,
        vec![
            Number {
                row_index: 0,
                col_from_index: 0,
                col_to_index: 2,
                value: 467,
            },
            Number {
                row_index: 0,
                col_from_index: 9,
                col_to_index: 9,
                value: 2,
            },
        ]
    );
}

#[test]
fn should_extract_symbols_from_a_line() {
    let input = "..$..+*%&#";
    let line_number = 0;

    let symbols = extract_symbols_from_line(line_number, input);

    assert_eq!(
        symbols,
        vec![
            Symbol {
                row_index: 0,
                col_index: 2,
            },
            Symbol {
                row_index: 0,
                col_index: 5,
            },
            Symbol {
                row_index: 0,
                col_index: 6,
            },
            Symbol {
                row_index: 0,
                col_index: 7,
            },
            Symbol {
                row_index: 0,
                col_index: 8,
            },
            Symbol {
                row_index: 0,
                col_index: 9,
            },
        ]
    );
}

#[test]
fn should_filter_relevant_numbers() {
    let numbers = vec![
        Number {
            row_index: 0,
            col_from_index: 0,
            col_to_index: 2,
            value: 467,
        },
        Number {
            row_index: 0,
            col_from_index: 9,
            col_to_index: 9,
            value: 2,
        },
    ];

    let symbols = vec![
        Symbol {
            row_index: 1,
            col_index: 0,
        },
        Symbol {
            row_index: 2,
            col_index: 0,
        },
    ];

    let possible_numbers = possible_numbers(numbers, symbols);

    assert_eq!(
        possible_numbers,
        [Number {
            row_index: 0,
            col_from_index: 0,
            col_to_index: 2,
            value: 467,
        }]
    );
}

#[test]
fn should_create_schematic_from_lines() {
    let lines = vec!["467..114..", "...*......", "..35..633.", "......#..."];

    let schematic = Schematic::from_lines(lines);

    assert_eq!(schematic.numbers.len(), 4);
    assert_eq!(schematic.symbols.len(), 2);
}
