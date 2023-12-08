use std::fs;

fn main() {
    let path = std::path::Path::new("day-6/src/input.txt");
    let input = fs::read_to_string(path).expect("Should have been able to read the file input.txt");

    println!("Trying to solve part 1...");
    let records = day_6::records(input.lines().collect());

    println!("Solution is: {}", records);

    println!("Trying to solve part 2...");
    let path_2 = std::path::Path::new("day-6/src/input-2.txt");
    let input_2 =
        fs::read_to_string(path_2).expect("Should have been able to read the file input.txt");

    let before = std::time::Instant::now();
    let records_2 = day_6::records(input_2.lines().collect());
    let after = std::time::Instant::now();

    println!("Time taken: {:?}", after.duration_since(before));
    println!("Solution is: {}", records_2);
}
