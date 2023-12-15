use std::fs;

fn main() {
    let path = std::path::Path::new("day-11/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file day-1.txt");

    println!("Trying to solve part 1...");
    let path = day_11::sum_of_shortest_path_between_galaxies(input.lines().collect());
    println!("Solution is: {}", path);
}
