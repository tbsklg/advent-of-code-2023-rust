use day_3::sum_up_possible_numbers;

#[test]
fn should_calculate_sum_of_part_numbers() {
    let input = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    let sum_of_part_numbers = sum_up_possible_numbers(input);

    assert_eq!(sum_of_part_numbers, 4361);
}

#[test]
fn should_calculate_gear_ratios() {
    let input = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    let sum_of_all_gear_ratios = day_3::sum_of_all_gear_ratios(input);

    assert_eq!(sum_of_all_gear_ratios, 467835);
}
