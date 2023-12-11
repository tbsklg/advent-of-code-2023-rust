#[test]
fn should_calc_extrapolated_values() {
    let input = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    let result = day_9::calc_extrapolated_values(input);

    assert_eq!(result, 114);
}
