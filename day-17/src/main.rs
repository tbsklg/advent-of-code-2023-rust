use std::fs;

fn main() {
    let path = std::path::Path::new("day-17/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let start = std::time::Instant::now();
    let tiles = day_17::heat_loss(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Solution is: {}", tiles);
    println!("Elapsed time: {:?}", elapsed);
}
