use std::fs;

fn main() {
    let path = std::path::Path::new("day-13/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let notes = day_13::notes(input.lines().collect());
    println!("Solution is: {}", notes);
}
