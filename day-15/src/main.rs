use std::fs;

fn main() {
    let path = std::path::Path::new("day-15/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let hash_sum = day_15::hash_sum(input.as_str());
    println!("Solution is: {}", hash_sum);

    println!("Trying to solve part 2...");
    let focusing_power = day_15::focusing_power(input.as_str());
    println!("Solution is: {}", focusing_power);
}
