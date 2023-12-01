use std::fs;

fn main() {
    println!("Hello to the Advent of Code 2023 Day 1 Challenge!");

    let input = fs::read_to_string("../resources/day-1.txt")
        .expect("Should have been able to read the file day-1.txt");

    println!("Trying to calculate the calibration value for provided input (part 1)...");
    let calibration_value_one = day_1::calculate_calibration_value(input.lines().collect());
    println!("Calibration value is: {}", calibration_value_one);

    println!("Trying to calculate the calibration value for provided input (part 2)...");
    let calibration_value_two =
        day_1::calculate_calibration_value_with_letters(input.lines().collect());
    println!("Calibration value is: {}", calibration_value_two);
}
