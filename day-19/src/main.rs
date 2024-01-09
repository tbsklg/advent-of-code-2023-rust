use std::fs;

fn main() {
    let path = std::path::Path::new("day-19/src/sample.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let start = std::time::Instant::now();
    let tiles = day_19::rating(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Solution is: {}", tiles);

    println!("Trying to solve part 2...");
    let start = std::time::Instant::now();
    let tiles = day_19::combinations(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Solution is: {}", tiles);
}
