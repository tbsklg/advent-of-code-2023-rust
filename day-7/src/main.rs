use std::fs;

fn main() {
    let path = std::path::Path::new("day-7/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let winnings = day_7::calculate_total_winnings(input.lines().collect());

    println!("Solution is: {}", winnings);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let winnings_with_joker = day_7::calculate_total_winnings_with_joker(input.lines().collect());

    let after = std::time::Instant::now();
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", winnings_with_joker);
}
