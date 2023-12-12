fn main() {
    let path = std::path::Path::new("day-9/src/input.txt");
    let input =
        std::fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let extrapolated_values_right =
        day_9::calc_extrapolated_values_to_right(input.lines().collect());

    println!("Solution is: {}", extrapolated_values_right);

    let input =
        std::fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 2...");
    let extrapolated_values_left = day_9::calc_extrapolated_values_to_left(input.lines().collect());

    println!("Solution is: {}", extrapolated_values_left);
}
