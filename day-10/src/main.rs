use std::fs;

fn main() {
    let path = std::path::Path::new("day-10/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file day-1.txt");

    println!("Trying to solve part 1...");
    let steps = day_10::calculate_steps(input.lines().collect());
    println!("Solution is: {}", steps);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let enclosing_points = day_10::calculate_number_of_enclosing_points(input.lines().collect());
    let after = std::time::Instant::now();
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", enclosing_points);
}
