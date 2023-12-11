fn main() {
    let path = std::path::Path::new("day-9/src/input.txt");
    let input =
        std::fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let sum_of_extrapolated_values = day_9::calc_extrapolated_values(input.lines().collect());

    println!("Solution is: {}", sum_of_extrapolated_values);
}
