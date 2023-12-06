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
        day_5::calculate_lowest_location_number(input.lines().collect(), strategy);

    println!("Solution is: {}", location_number);
}
