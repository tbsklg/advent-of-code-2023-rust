#[test]
fn should_caculate_records() {
    let input = vec!["Time:      7  15   30", "Distance:  9  40  200"];

    let result = day_6::records(input);

    assert_eq!(result, 288);
}
