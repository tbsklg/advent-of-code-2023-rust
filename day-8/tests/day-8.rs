#[test]
fn should_calculate_steps() {
    let input = vec![
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    let steps = day_8::calculate_steps(input, "AAA", "ZZZ");

    assert_eq!(steps, 6);
}

#[test]
fn should_calculate_steps_with_multiple_starting_nodes() {
    let input = vec![
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    let steps = day_8::calcualte_steps_with_multiple_starts(input);

    assert_eq!(steps, 6);
}
