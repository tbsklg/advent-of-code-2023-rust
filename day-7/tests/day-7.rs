#[test]
fn should_calculate_winnings() {
    let input = vec![
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    let winnings = day_7::calculate_total_winnings(input);

    assert_eq!(winnings, 6440);
}
