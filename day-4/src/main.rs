use std::fs;

fn main() {
    let path = std::path::Path::new("day-4/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let points = day_4::calculate_points(input.lines().collect());

    println!("Solution is: {}", points);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let cards = day_4::calculate_scratchcards(input.lines().collect());

    let after = std::time::Instant::now();
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", cards);
}
