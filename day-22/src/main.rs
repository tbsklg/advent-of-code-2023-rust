use std::fs;

fn main() {
    let path = std::path::Path::new("day-22/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let start = std::time::Instant::now();
    let result = day_22::bricks(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Solution is: {:?}", result);

    println!("Trying to solve part 2...");
    let start = std::time::Instant::now();
    let result = day_22::all_bricks(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Solution is: {:?}", result);
}
