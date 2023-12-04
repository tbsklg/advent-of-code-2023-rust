use std::fs;

fn main() {
    let path = std::path::Path::new("day-3/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let sum_of_numbers = day_3::sum_up_possible_numbers(input.lines().collect());

    println!("Solution is: {}", sum_of_numbers);
}
