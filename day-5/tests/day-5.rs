use day_5::Category;

#[test]
fn should_lowest_location_number() {
    let input = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    let strategy = vec![
        Category::SOIL,
        Category::FERTILIZER,
        Category::WATER,
        Category::LIGHT,
        Category::TEMPERATURE,
        Category::HUMIDITY,
        Category::LOCATION,
    ];
    let points = day_5::calculate_lowest_location_number(input, strategy);

    assert_eq!(points, 35);
}

#[test]
fn should_lowest_location_number_with_ranges() {
    let input = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4",
    ];

    let strategy = vec![
        Category::SOIL,
        Category::FERTILIZER,
        Category::WATER,
        Category::LIGHT,
        Category::TEMPERATURE,
        Category::HUMIDITY,
        Category::LOCATION,
    ];
    let points = day_5::calculate_lowest_location_number_with_ranges(input, strategy);

    assert_eq!(points, 46);
}
