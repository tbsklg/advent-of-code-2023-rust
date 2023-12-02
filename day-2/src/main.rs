use std::{collections::HashMap, fs};

use day_2::Color;

fn main() {
    let path = std::path::Path::new("day-2/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let sum_of_ids = day_2::calculate_sum_of_game_ids(
        input.lines().collect(),
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]),
    );

    println!("Solution is: {}", sum_of_ids);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let power_of_cube_sets =
        day_2::calculate_sum_of_the_power_of_cube_sets(input.lines().collect());

    let after = std::time::Instant::now();
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", power_of_cube_sets);
}
