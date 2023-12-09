use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Clone, PartialOrd, Ord, Eq, Hash)]
struct Hand {
    cards: String,
    bid: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn new(input: &str) -> Self {
        let cards = input.split(' ').nth(0).unwrap();
        let bid = input.split(' ').nth(1).unwrap().parse::<u32>().unwrap();

        Self {
            cards: cards.to_string(),
            bid,
        }
    }
}

pub fn calculate_total_winnings(input: Vec<&str>) -> u32 {
    let mut hands = input.iter().map(|x| Hand::new(x)).collect::<Vec<Hand>>();

    hands.sort_by(|a, b| sort_hand_type(a.clone(), b.clone()).unwrap());

    hands.iter().enumerate().fold(0, |mut acc, (i, x)| {
        acc += x.bid * (i + 1) as u32;
        acc
    })
}

pub fn calculate_total_winnings_with_joker(input: Vec<&str>) -> u32 {
    let mut hands = input.iter().map(|x| Hand::new(x)).collect::<Vec<Hand>>();

    hands.sort_by(|a, b| sort_hand_type_with_joker(a.clone(), b.clone()).unwrap());

    hands.iter().enumerate().fold(0, |mut acc, (i, x)| {
        acc += x.bid * (i + 1) as u32;
        acc
    })
}

fn sort_hand_type_with_joker(first: Hand, second: Hand) -> Option<Ordering> {
    let first_hand_type = max_hand_type_with_joker(first.clone());
    let second_hand_type = max_hand_type_with_joker(second.clone());

    first_hand_type
        .partial_cmp(&second_hand_type)
        .and_then(|x| match x {
            Ordering::Equal => sort_cards(first, second, "J23456789TQKA"),
            _ => Some(x),
        })
}

fn sort_hand_type(first: Hand, second: Hand) -> Option<Ordering> {
    let first_hand_type = hand_type(first.clone());
    let second_hand_type = hand_type(second.clone());

    first_hand_type
        .partial_cmp(&second_hand_type)
        .and_then(|x| match x {
            Ordering::Equal => sort_cards(first, second, "23456789TJQKA"),
            _ => Some(x),
        })
}

fn max_hand_type_with_joker(hand: Hand) -> HandType {
    "23456789TJQKA"
        .chars()
        .map(|x| {
            let mut hand = hand.clone();
            hand.cards = hand.cards.replace('J', &x.to_string());
            hand_type(hand)
        })
        .max()
        .unwrap()
}

fn sort_cards(first: Hand, second: Hand, card_order: &str) -> Option<Ordering> {
    let first_card = first
        .cards
        .chars()
        .map(|x| card_order.find(x).unwrap())
        .collect::<Vec<usize>>();

    let second_card = second
        .cards
        .chars()
        .map(|x| card_order.find(x).unwrap())
        .collect::<Vec<usize>>();

    Some(first_card.partial_cmp(&second_card).unwrap())
}

fn hand_type(hand: Hand) -> HandType {
    let counts = hand.cards.chars().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    match counts.values().max() {
        Some(5) => HandType::FiveOfAKind,
        Some(4) => HandType::FourOfAKind,
        Some(3) => match counts.values().any(|x| *x == 2) {
            true => HandType::FullHouse,
            false => HandType::ThreeOfAKind,
        },
        Some(2) => match counts.values().filter(|x| **x == 2).count() == 2 {
            true => HandType::TwoPair,
            false => HandType::OnePair,
        },
        _ => HandType::HighCard,
    }
}

#[test]
fn should_parse_hand_with_one_pair() {
    let input = "32T3K 765";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: String::from("32T3K"),
            bid: 765,
        }
    );

    assert_eq!(hand_type(hand), HandType::OnePair);
}

#[test]
fn should_parse_hand_with_two_pairs() {
    let input = "32K3K 765";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: String::from("32K3K"),
            bid: 765,
        }
    );

    assert_eq!(hand_type(hand), HandType::TwoPair);
}

#[test]
fn should_parse_hand_with_three_of_a_kind() {
    let input = "T55J5 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: String::from("T55J5"),
            bid: 65,
        }
    );

    assert_eq!(hand_type(hand), HandType::ThreeOfAKind);
}

#[test]
fn should_parse_hand_with_four_of_a_kind() {
    let input = "5555J 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: String::from("5555J"),
            bid: 65,
        }
    );

    assert_eq!(hand_type(hand), HandType::FourOfAKind);
}

#[test]
fn should_parse_hand_with_five_of_a_kind() {
    let input = "55555 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: String::from("55555"),
            bid: 65,
        }
    );

    assert_eq!(hand_type(hand), HandType::FiveOfAKind);
}

#[test]
fn should_parse_hand_with_full_house() {
    let input = "555JJ 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: String::from("555JJ"),
            bid: 65,
        }
    );

    assert_eq!(hand_type(hand), HandType::FullHouse);
}

#[test]
fn should_parse_hand_with_high_card() {
    let input = "AKQJT 65";
    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: String::from("AKQJT"),
            bid: 65,
        }
    );

    assert_eq!(hand_type(hand), HandType::HighCard);
}

#[test]
fn should_sort_hands_with_different_types() {
    let full_house = Hand {
        cards: String::from("555JJ"),
        bid: 65,
    };

    let high_card = Hand {
        cards: String::from("AKQJT"),
        bid: 65,
    };

    let mut hands = vec![full_house.clone(), high_card.clone()];
    hands.sort_by(|a, b| sort_hand_type(a.clone(), b.clone()).unwrap());

    assert_eq!(hands, vec![high_card, full_house]);
}

#[test]
fn should_sort_hands_with_same_type_and_different_cards() {
    let full_house_1 = Hand {
        cards: String::from("555JJ"),
        bid: 65,
    };

    let full_house_2 = Hand {
        cards: String::from("555TT"),
        bid: 65,
    };

    let mut hands = vec![full_house_1.clone(), full_house_2.clone()];

    hands.sort_by(|a, b| sort_hand_type(a.clone(), b.clone()).unwrap());

    assert_eq!(hands, vec![full_house_2, full_house_1]);
}

#[test]
fn should_sort_hands_with_high_cards() {
    let high_card_six = Hand {
        cards: String::from("AKQJT"),
        bid: 65,
    };

    let high_card_seven = Hand {
        cards: String::from("AKQJT"),
        bid: 65,
    };

    let high_card_eight = Hand {
        cards: String::from("AKQJT"),
        bid: 65,
    };

    let mut hands = vec![
        high_card_six.clone(),
        high_card_eight.clone(),
        high_card_seven.clone(),
    ];

    hands.sort();

    assert_eq!(hands, vec![high_card_six, high_card_seven, high_card_eight]);
}

#[test]
fn should_sort_hands_with_same_type() {
    let four_of_a_kind_1 = Hand {
        cards: String::from("5555J"),
        bid: 65,
    };

    let four_of_a_kind_2 = Hand {
        cards: String::from("5555J"),
        bid: 65,
    };

    let mut hands = vec![four_of_a_kind_1.clone(), four_of_a_kind_2.clone()];

    hands.sort();

    assert_eq!(hands, vec![four_of_a_kind_2, four_of_a_kind_1]);
}

#[test]
fn should_sort_hands_with_both_have_a_full_house() {
    let full_house_1 = Hand {
        cards: String::from("555JJ"),
        bid: 65,
    };

    let full_house_2 = Hand {
        cards: String::from("555JJ"),
        bid: 65,
    };

    let mut hands = vec![full_house_1.clone(), full_house_2.clone()];

    hands.sort();

    assert_eq!(hands, vec![full_house_2, full_house_1]);
}

#[test]
fn should_find_max_hand_type_with_joker() {
    let hand = Hand {
        cards: String::from("KTJJT"),
        bid: 65,
    };

    assert_eq!(max_hand_type_with_joker(hand), HandType::FourOfAKind);
}
