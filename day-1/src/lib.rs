pub fn calculate_calibration_value(input: Vec<&str>) -> u32 {
    extract_numbers(input).iter().map(|x| *x as u32).sum()
}

pub fn calculate_calibration_value_with_letters(input: Vec<&str>) -> u32 {
    extract_numbers_with_letters(input)
        .iter()
        .map(|x| *x as u32)
        .sum()
}

fn extract_numbers(input: Vec<&str>) -> Vec<i8> {
    input.iter().map(|x| extract_number(x)).collect()
}

fn extract_numbers_with_letters(input: Vec<&str>) -> Vec<i8> {
    input
        .iter()
        .map(|x| extract_number_with_letters(x))
        .collect()
}

fn extract_number(str: &str) -> i8 {
    find_digits(str)
        .and_then(|x| get_first_and_last(&x))
        .and_then(|x| parse_i8(&x).ok())
        .unwrap_or(0)
}

fn extract_number_with_letters(str: &str) -> i8 {
    extract_number(replace_letter_with_digit(str).as_str())
}

fn find_digits(str: &str) -> Option<String> {
    let digits = str.chars().filter(|x| x.is_digit(10)).collect();

    match digits == "" {
        true => None,
        false => Some(digits),
    }
}

const LETTERS_TO_DIGITS: [(&str, i8); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn replace_letter_with_digit(input: &str) -> String {
    if input.is_empty() {
        String::from("")
    } else {
        LETTERS_TO_DIGITS
            .iter()
            .find(|(letter, _)| input.starts_with(letter))
            .map(|(_, digit)| digit.to_string() + replace_letter_with_digit(&input[1..]).as_str())
            .unwrap_or(
                input.chars().next().unwrap().to_string()
                    + replace_letter_with_digit(&input[1..]).as_str(),
            )
    }
}

fn get_first_and_last(str: &str) -> Option<String> {
    let mut chars = str.chars().into_iter();
    let head = chars.next();
    let last = chars.last();

    match (head, last) {
        (Some(h), Some(l)) => Some(format!("{}{}", h, l)),
        (Some(h), None) => Some(format!("{}{}", h, h)),
        _ => None,
    }
}

fn parse_i8(str: &str) -> Result<i8, String> {
    str.parse::<i8>()
        .map_err(|_| String::from("Invalid string provided"))
}

#[test]
fn should_find_digits_in_string() {
    let input = "pqr3stu8vwx";

    let digits = find_digits(input);

    assert_eq!(digits, Some(String::from("38")));
}

#[test]
fn should_find_no_digits_in_string() {
    let input = "pqrstuvwx";

    let digits = find_digits(input);

    assert_eq!(digits, None);
}

#[test]
fn should_return_first_and_last_char() {
    let input = "ab1cd";

    let first_and_last = get_first_and_last(input);

    assert_eq!(first_and_last, Some(String::from("ad")));
}

#[test]
fn should_return_first_twice_for_one_char() {
    let input = "a";

    let first_and_last = get_first_and_last(input);

    assert_eq!(first_and_last, Some(String::from("aa")));
}

#[test]
fn should_parse_a_string_to_i8() {
    let input = "12";

    let number = parse_i8(input);

    assert_eq!(number, Ok(12));
}

#[test]
fn should_return_error_for_invalid_string() {
    let input = "12a";

    let number = parse_i8(input);

    assert_eq!(number, Err(String::from("Invalid string provided")));
}

#[test]
fn should_extract_number_from_string() {
    let input = "pqr3stu8vwx";

    let digit = extract_number(input);

    assert_eq!(digit, 38);
}

#[test]
fn should_return_zero_for_no_digit_within_string() {
    let input = "abc";

    let digit = extract_number(input);

    assert_eq!(digit, 0);
}

#[test]
fn should_return_at_least_two_digits() {
    let input = "a2c";

    let digit = extract_number(input);

    assert_eq!(digit, 22);
}

#[test]
fn should_convert_a_list_of_strings_into_digits() {
    let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    let numbers = extract_numbers(input);

    assert_eq!(numbers, vec![12, 38, 15, 77]);
}

#[test]
fn should_convert_a_list_of_strings_containing_no_digits_into_digits() {
    let input = vec!["abc", "pqrtuvwx", "abcdef", "treb7uchet"];

    let numbers = extract_numbers(input);

    assert_eq!(numbers, vec![0, 0, 0, 77]);
}

#[test]
fn should_extract_numbers_provided_as_letters() {
    let input = "two1nine";

    let number = extract_number_with_letters(input);

    assert_eq!(number, 29);
}

#[test]
fn should_extract_number_from_eightwothree() {
    let input = "eightwothree";

    let number = extract_number_with_letters(input);

    assert_eq!(number, 83);
}

#[test]
fn should_replace_letter_with_digit() {
    let input = "one";

    let digit = replace_letter_with_digit(input);

    assert_eq!(digit, "1ne");
}
