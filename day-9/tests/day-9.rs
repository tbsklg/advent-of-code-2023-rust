#[test]
fn should_calc_extrapolated_values_to_right() {
    let input = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    let result = day_9::calc_extrapolated_values_to_right(input);

    assert_eq!(result, 114);
}

#[test]
fn should_calc_extrapolated_values_left() {
    let input = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    let result = day_9::calc_extrapolated_values_to_left(input);

    assert_eq!(result, 2);
}
