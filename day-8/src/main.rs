use std::fs;

fn main() {
    let path = std::path::Path::new("day-8/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let steps = day_8::calculate_steps(input.lines().collect(), "AAA", "ZZZ");

    println!("Solution is: {}", steps);

    println!("Trying to solve part 2...");

    let before = std::time::Instant::now();
    let steps_with_multiple_starts =
        day_8::calcualte_steps_with_multiple_starts(input.lines().collect());
    let after = std::time::Instant::now();

    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", steps_with_multiple_starts);
}
