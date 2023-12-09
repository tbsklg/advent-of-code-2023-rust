use std::fs;

fn main() {
    let path = std::path::Path::new("day-7/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let winnings = day_7::calculate_total_winnings(input.lines().collect());

    println!("Solution is: {}", winnings);
}
