#[test]
fn should_sum_up_calibration_numbers() {
    let input = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

    let sum_of_calibration_values = day_1::calculate_calibration_value(input);

    assert_eq!(sum_of_calibration_values, 142);
}

#[test]
fn should_sum_up_calibration_numbers_with_letters() {
    let input = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    let sum_of_calibration_values = day_1::calculate_calibration_value_with_letters(input);

    assert_eq!(sum_of_calibration_values, 281);
}
