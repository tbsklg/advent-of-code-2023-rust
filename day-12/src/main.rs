use std::fs;

fn main() {
    let path = std::path::Path::new("day-12/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file day-1.txt");

    println!("Trying to solve part 1...");
    let arrangements = day_12::arrangements(input.lines().collect());
    println!("Solution is: {}", arrangements);

    println!("Trying to solve part 2...");
    let arrangements_five = day_12::arrangements_five(input.lines().collect());
    println!("Solution is: {}", arrangements_five);
}
