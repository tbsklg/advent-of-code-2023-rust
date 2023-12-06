use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Category {
    SOIL,
    FERTILIZER,
    WATER,
    LIGHT,
    TEMPERATURE,
    HUMIDITY,
    LOCATION,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Conversion {
    source: u64,
    dest: u64,
    range: u64,
}

pub fn calculate_lowest_location_number(input: Vec<&str>, strategy: Vec<Category>) -> u64 {
    let (seeds, maps) = parse_input(input);

    seeds
        .into_iter()
        .map(|s| calculate_conversions(s, maps.clone(), strategy.clone()))
        .min()
        .unwrap()
}

fn calculate_conversions(
    source: u64,
    map: HashMap<Category, Vec<Conversion>>,
    strategy: Vec<Category>,
) -> u64 {
    match strategy.is_empty() {
        true => source,
        false => {
            let maybe_next = strategy
                .get(0)
                .and_then(|s| map.get(s))
                .and_then(|m| calculate_conversion(source, m.to_vec()));

            calculate_conversions(
                maybe_next.unwrap(),
                map,
                strategy.into_iter().skip(1).collect(),
            )
        }
    }
}

fn calculate_conversion(input: u64, map: Vec<Conversion>) -> Option<u64> {
    let relevant_conversion = map.iter().find(|conversion| {
        input >= conversion.source && input <= conversion.source + conversion.range - 1
    });

    match relevant_conversion {
        Some(conversion) => Some(input + conversion.dest - conversion.source),
        _ => Some(input),
    }
}

fn parse_conversion(input: &str) -> Option<Conversion> {
    let raw_parts: Vec<&str> = input.split(' ').collect();

    match (raw_parts.get(0), raw_parts.get(1), raw_parts.get(2)) {
        (Some(dest), Some(source), Some(range)) => Some(Conversion {
            source: source.parse::<u64>().unwrap(),
            dest: dest.parse::<u64>().unwrap(),
            range: range.parse::<u64>().unwrap(),
        }),
        _ => None,
    }
}

lazy_static! {
    static ref SPLIT_CATEGORY_REGEX: Regex = Regex::new(r"[ -]").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

fn parse_seeds(input: &str) -> Vec<u64> {
    NUMBER_REGEX
        .find_iter(input)
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect()
}

fn parse_category(input: &str) -> Option<Category> {
    let title: Vec<&str> = SPLIT_CATEGORY_REGEX.split(input).collect();

    map_category(title.get(2).unwrap())
}

fn map_category(input: &str) -> Option<Category> {
    match input {
        "soil" => Some(Category::SOIL),
        "fertilizer" => Some(Category::FERTILIZER),
        "water" => Some(Category::WATER),
        "light" => Some(Category::LIGHT),
        "temperature" => Some(Category::TEMPERATURE),
        "humidity" => Some(Category::HUMIDITY),
        "location" => Some(Category::LOCATION),
        _ => None,
    }
}

fn parse_map(input: Vec<&str>) -> Option<(Category, Vec<Conversion>)> {
    let maybe_category = input.get(0).and_then(|x| parse_category(x));
    let maybe_conversions = input.iter().skip(1).map(|x| parse_conversion(x)).collect();

    match (maybe_category, maybe_conversions) {
        (Some(category), Some(conversions)) => Some((category, conversions)),
        _ => None,
    }
}

fn parse_maps(input: Vec<Vec<&str>>) -> Option<HashMap<Category, Vec<Conversion>>> {
    let maybe_maps: Option<Vec<(Category, Vec<Conversion>)>> =
        input.into_iter().map(|x| parse_map(x)).collect();

    match maybe_maps {
        Some(maps) => Some(maps.iter().fold(HashMap::new(), |mut acc, curr| {
            acc.insert(curr.0, curr.1.clone());
            acc
        })),
        _ => None,
    }
}

fn parse_raw_maps(input: Vec<&str>) -> Option<HashMap<Category, Vec<Conversion>>> {
    let raw_maps: Vec<Vec<&str>> = input.split(|x| x.is_empty()).map(|x| x.to_vec()).collect();

    parse_maps(raw_maps)
}

fn parse_input(input: Vec<&str>) -> (Vec<u64>, HashMap<Category, Vec<Conversion>>) {
    let seeds = parse_seeds(input.get(0).unwrap());
    let maps = parse_raw_maps(input.into_iter().skip(2).collect()).unwrap();

    (seeds, maps)
}

#[test]
fn should_parse_almanac() {
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
    ];

    let almanac = parse_input(input);

    assert_eq!(
        almanac,
        (
            vec![79, 14, 55, 13],
            HashMap::from([
                (
                    Category::SOIL,
                    vec![
                        Conversion {
                            source: 98,
                            dest: 50,
                            range: 2,
                        },
                        Conversion {
                            source: 50,
                            dest: 52,
                            range: 48,
                        },
                    ],
                ),
                (
                    Category::FERTILIZER,
                    vec![
                        Conversion {
                            source: 15,
                            dest: 0,
                            range: 37,
                        },
                        Conversion {
                            source: 52,
                            dest: 37,
                            range: 2,
                        },
                        Conversion {
                            source: 0,
                            dest: 39,
                            range: 15,
                        },
                    ],
                ),
            ])
        )
    )
}

#[test]
fn should_parse_seeds_line() {
    let input = "seeds: 79 14 55 13";

    let seeds = parse_seeds(input);

    assert_eq!(seeds, vec![79, 14, 55, 13]);
}

#[test]
fn should_parse_a_list_of_maps() {
    let input = vec![
        vec!["seed-to-soil map:", "50 98 2", "52 50 48"],
        vec!["fertilizer-to-water map", "49 53 8", "0 11 42"],
    ];

    let maps = parse_maps(input);

    assert_eq!(
        maps,
        Some(HashMap::from([
            (
                Category::SOIL,
                vec![
                    Conversion {
                        source: 98,
                        dest: 50,
                        range: 2,
                    },
                    Conversion {
                        source: 50,
                        dest: 52,
                        range: 48,
                    }
                ]
            ),
            (
                Category::WATER,
                vec![
                    Conversion {
                        source: 53,
                        dest: 49,
                        range: 8,
                    },
                    Conversion {
                        source: 11,
                        dest: 0,
                        range: 42,
                    },
                ]
            ),
        ]))
    );
}

#[test]
fn should_parse_seed_to_soil_map() {
    let input = vec!["seed-to-soil map:", "50 98 2", "52 50 48"];

    let seed_to_soil = parse_map(input);

    assert_eq!(
        seed_to_soil,
        Some((
            Category::SOIL,
            vec![
                Conversion {
                    source: 98,
                    dest: 50,
                    range: 2,
                },
                Conversion {
                    source: 50,
                    dest: 52,
                    range: 48,
                }
            ],
        ))
    );
}

#[test]
fn should_calculate_conversion() {
    let input = 79;
    let map = vec![
        Conversion {
            source: 98,
            dest: 50,
            range: 2,
        },
        Conversion {
            source: 50,
            dest: 52,
            range: 48,
        },
    ];

    let conversion = calculate_conversion(input, map);

    assert_eq!(conversion, Some(81));
}

#[test]
fn should_return_input_if_no_range_exists() {
    let input = 14;
    let map = vec![
        Conversion {
            source: 98,
            dest: 50,
            range: 2,
        },
        Conversion {
            source: 50,
            dest: 52,
            range: 48,
        },
    ];

    let conversion = calculate_conversion(input, map);

    assert_eq!(conversion, Some(14));
}

#[test]
fn should_calculate_conversions_based_on_corresponding_types() {
    let seed = 14;
    let map = HashMap::from([
        (
            Category::SOIL,
            vec![
                Conversion {
                    source: 98,
                    dest: 50,
                    range: 2,
                },
                Conversion {
                    source: 50,
                    dest: 52,
                    range: 48,
                },
            ],
        ),
        (
            Category::FERTILIZER,
            vec![
                Conversion {
                    source: 15,
                    dest: 0,
                    range: 37,
                },
                Conversion {
                    source: 52,
                    dest: 37,
                    range: 2,
                },
                Conversion {
                    source: 0,
                    dest: 39,
                    range: 15,
                },
            ],
        ),
    ]);

    let strategy = vec![Category::SOIL, Category::FERTILIZER];

    let result = calculate_conversions(seed, map, strategy);

    assert_eq!(result, 53);
}

#[test]
fn should_parse_an_invalid_conversion() {
    let input = "50 98";

    let conversion = parse_conversion(input);

    assert_eq!(conversion, None,);
}

#[test]
fn should_parse_category() {
    let input = "seed-to-soil map:";

    let category = parse_category(input);

    assert_eq!(category, Some(Category::SOIL));
}

#[test]
fn should_parse_fertilizer_category() {
    let input = "soil-to-fertilizer map";

    let category = parse_category(input);

    assert_eq!(category, Some(Category::FERTILIZER));
}

#[test]
fn should_parse_water_category() {
    let input = "fertilizer-to-water map";

    let category = parse_category(input);

    assert_eq!(category, Some(Category::WATER));
}

#[test]
fn should_parse_light_category() {
    let input = "water-to-light map";

    let category = parse_category(input);

    assert_eq!(category, Some(Category::LIGHT));
}

#[test]
fn should_parse_temperature_category() {
    let input = "light-to-temperature map";

    let category = parse_category(input);

    assert_eq!(category, Some(Category::TEMPERATURE));
}

#[test]
fn should_parse_humidity() {
    let input = "light-to-humidity map";

    let category = parse_category(input);

    assert_eq!(category, Some(Category::HUMIDITY));
}

#[test]
fn should_parse_a_conversion() {
    let input = "50 98 2";

    let conversion = parse_conversion(input);

    assert_eq!(
        conversion,
        Some(Conversion {
            source: 98,
            dest: 50,
            range: 2,
        }),
    );
}
