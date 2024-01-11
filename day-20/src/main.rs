use std::fs;

fn main() {
    let path = std::path::Path::new("day-20/src/sample.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file");

    println!("Trying to solve part 1...");
    let start = std::time::Instant::now();
    let result = day_20::pulses(input.lines().collect());
    let elapsed = start.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    println!("Solution is: {:?}", result);
}
