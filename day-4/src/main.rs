use std::fs;

fn main() {
    let path = std::path::Path::new("day-4/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let points = day_4::calculate_points(input.lines().collect());

    println!("Solution is: {}", points);
}
