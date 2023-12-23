use std::fs;

fn main() {
    let path = std::path::Path::new("day-14/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let rounded_rocks = day_14::tilt_north(input.lines().collect());
    println!("Solution is: {}", rounded_rocks);
}
