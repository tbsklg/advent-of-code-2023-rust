use std::fs;

fn main() {
    let path = std::path::Path::new("day-3/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let sum_of_numbers = day_3::sum_up_possible_numbers(input.lines().collect());

    println!("Solution is: {}", sum_of_numbers);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let sum_of_gear_ratios = day_3::sum_of_all_gear_ratios(input.lines().collect());

    let after = std::time::Instant::now();
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", sum_of_gear_ratios);
}
