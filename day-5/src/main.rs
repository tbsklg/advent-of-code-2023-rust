use std::fs;

use day_5::Category;

fn main() {
    let path = std::path::Path::new("day-5/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");

    let strategy = vec![
        Category::SOIL,
        Category::FERTILIZER,
        Category::WATER,
        Category::LIGHT,
        Category::TEMPERATURE,
        Category::HUMIDITY,
        Category::LOCATION,
    ];

    let location_number =
        day_5::calculate_lowest_location_number(input.lines().collect(), strategy.clone());

    println!("Solution is: {}", location_number);

    println!("Trying to solve part 2...");
    let before = std::time::Instant::now();
    let location_number_ranges =
        day_5::calculate_lowest_location_number_with_ranges(input.lines().collect(), strategy);

    let after = std::time::Instant::now();
    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", location_number_ranges);
}
