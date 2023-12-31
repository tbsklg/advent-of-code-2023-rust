use std::fs;

fn main() {
    let path = std::path::Path::new("day-1/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file day-1.txt");

    println!("Trying to solve part 1...");
    let calibration_value_one = day_1::calculate_calibration_value(input.lines().collect());
    println!("Solution is: {}", calibration_value_one);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let calibration_value_two =
        day_1::calculate_calibration_value_with_letters(input.lines().collect());
    let after = std::time::Instant::now();
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", calibration_value_two);
}
