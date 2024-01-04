use std::fs;

fn main() {
    let path = std::path::Path::new("day-18/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let start = std::time::Instant::now();
    let tiles = day_18::cubic_meters(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Solution is: {}", tiles);
    println!("Elapsed time: {:?}", elapsed);

    println!("Trying to solve part 2...");
    let start = std::time::Instant::now();
    let tiles = day_18::cubic_meters_rgb(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Solution is: {}", tiles);
    println!("Elapsed time: {:?}", elapsed);
}
