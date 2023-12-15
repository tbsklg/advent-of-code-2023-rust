use std::fs;

fn main() {
    let path = std::path::Path::new("day-11/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file day-1.txt");

    println!("Trying to solve part 1...");
    let path = day_11::sum_of_shortest_path_between_galaxies(input.lines().collect(), 2);
    println!("Solution is: {}", path);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let after = std::time::Instant::now();
    let path = day_11::sum_of_shortest_path_between_galaxies(input.lines().collect(), 1_000_000);
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", path);
}
