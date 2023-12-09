use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Ord, Eq, Hash)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
enum CardLabel {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
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

#[derive(Debug, PartialEq, Ord, Eq, Clone, Hash)]
struct Card {
    label: CardLabel,
}

impl Card {
    fn from(c: char) -> Self {
        Self {
            label: map_label(c),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.label.partial_cmp(&other.label)
    }
}

fn map_label(c: char) -> CardLabel {
    match c {
        'A' => CardLabel::A,
        'K' => CardLabel::K,
        'Q' => CardLabel::Q,
        'J' => CardLabel::J,
        'T' => CardLabel::T,
        '9' => CardLabel::Nine,
        '8' => CardLabel::Eight,
        '7' => CardLabel::Seven,
        '6' => CardLabel::Six,
        '5' => CardLabel::Five,
        '4' => CardLabel::Four,
        '3' => CardLabel::Three,
        '2' => CardLabel::Two,
        _ => panic!("The provided card label is not recognized"),
    }
}

impl Hand {
    fn new(input: &str) -> Self {
        let cards = input.split(' ').nth(0).unwrap();
        let bid = input.split(' ').nth(1).unwrap().parse::<u32>().unwrap();

        Self {
            cards: cards.chars().map(Card::from).collect(),
            bid,
        }
    }

    fn hand_type(&self) -> HandType {
        if self.has_five_of_a_kind() {
            HandType::FiveOfAKind
        } else if self.has_four_of_a_kind() {
            HandType::FourOfAKind
        } else if self.has_full_house() {
            HandType::FullHouse
        } else if self.has_three_of_a_kind() {
            HandType::ThreeOfAKind
        } else if self.has_two_pairs() {
            HandType::TwoPair
        } else if self.has_one_pair() {
            HandType::OnePair
        } else if self.has_high_card() {
            HandType::HighCard
        } else {
            panic!("The hand type is not recognized")
        }
    }

    fn card_map(&self) -> HashMap<Card, u32> {
        self.cards.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x.clone()).or_insert(0) += 1;
            acc
        })
    }

    fn has_one_pair(&self) -> bool {
        self.card_map().values().filter(|x| **x == 2).count() == 1
    }

    fn has_two_pairs(&self) -> bool {
        self.card_map().values().filter(|x| **x == 2).count() == 2
    }

    fn has_three_of_a_kind(&self) -> bool {
        self.card_map().values().filter(|x| **x == 3).count() == 1
    }

    fn has_five_of_a_kind(&self) -> bool {
        self.card_map().values().filter(|x| **x == 5).count() == 1
    }

    fn has_four_of_a_kind(&self) -> bool {
        self.card_map().values().filter(|x| **x == 4).count() == 1
    }

    fn has_full_house(&self) -> bool {
        self.has_one_pair() && self.has_three_of_a_kind()
    }

    fn has_high_card(&self) -> bool {
        self.card_map().values().filter(|x| **x == 1).count() == 5
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();

        self_hand_type.partial_cmp(&other_hand_type).and_then(|x| {
            if x == std::cmp::Ordering::Equal {
                self.cards.partial_cmp(&other.cards)
            } else {
                Some(x)
            }
        })
    }
}

pub fn calculate_total_winnings(input: Vec<&str>) -> u32 {
    let mut hands = input.iter().map(|x| Hand::new(x)).collect::<Vec<Hand>>();

    hands.sort();

    hands.iter().enumerate().fold(0, |mut acc, (i, x)| {
        acc += x.bid * (i + 1) as u32;
        acc
    })
}

#[test]
fn should_parse_hand_with_one_pair() {
    let input = "32T3K 765";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: vec![
                Card {
                    label: CardLabel::Three,
                },
                Card {
                    label: CardLabel::Two,
                },
                Card {
                    label: CardLabel::T,
                },
                Card {
                    label: CardLabel::Three,
                },
                Card {
                    label: CardLabel::K,
                },
            ],
            bid: 765,
        }
    );

    assert_eq!(hand.hand_type(), HandType::OnePair);
}

#[test]
fn should_parse_hand_with_two_pairs() {
    let input = "32K3K 765";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: vec![
                Card {
                    label: CardLabel::Three,
                },
                Card {
                    label: CardLabel::Two,
                },
                Card {
                    label: CardLabel::K,
                },
                Card {
                    label: CardLabel::Three,
                },
                Card {
                    label: CardLabel::K,
                },
            ],
            bid: 765,
        }
    );

    assert_eq!(hand.hand_type(), HandType::TwoPair);
}

#[test]
fn should_parse_hand_with_three_of_a_kind() {
    let input = "T55J5 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: vec![
                Card {
                    label: CardLabel::T,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::J,
                },
                Card {
                    label: CardLabel::Five,
                },
            ],
            bid: 65,
        }
    );

    assert_eq!(hand.hand_type(), HandType::ThreeOfAKind);
}

#[test]
fn should_parse_hand_with_four_of_a_kind() {
    let input = "5555J 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: vec![
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::J,
                },
            ],
            bid: 65,
        }
    );

    assert_eq!(hand.hand_type(), HandType::FourOfAKind);
}

#[test]
fn should_parse_hand_with_five_of_a_kind() {
    let input = "55555 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: vec![
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
            ],
            bid: 65,
        }
    );

    assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
}

#[test]
fn should_parse_hand_with_full_house() {
    let input = "555JJ 65";

    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: vec![
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::Five,
                },
                Card {
                    label: CardLabel::J,
                },
                Card {
                    label: CardLabel::J,
                },
            ],
            bid: 65,
        }
    );

    assert_eq!(hand.hand_type(), HandType::FullHouse);
}

#[test]
fn should_parse_hand_with_high_card() {
    let input = "AKQJT 65";
    let hand = Hand::new(input);

    assert_eq!(
        hand,
        Hand {
            cards: vec![
                Card {
                    label: CardLabel::A,
                },
                Card {
                    label: CardLabel::K,
                },
                Card {
                    label: CardLabel::Q,
                },
                Card {
                    label: CardLabel::J,
                },
                Card {
                    label: CardLabel::T,
                },
            ],
            bid: 65,
        }
    );

    assert_eq!(hand.hand_type(), HandType::HighCard);
}

#[test]
fn should_parse_a_card() {
    let input = 'A';

    let card = Card::from(input);

    assert_eq!(
        card,
        Card {
            label: CardLabel::A,
        }
    );
}

#[test]
fn should_sort_cards() {
    let mut input = vec![
        Card {
            label: CardLabel::J,
        },
        Card {
            label: CardLabel::A,
        },
        Card {
            label: CardLabel::Two,
        },
        Card {
            label: CardLabel::K,
        },
    ];

    input.sort();

    assert_eq!(
        input,
        vec![
            Card {
                label: CardLabel::Two,
            },
            Card {
                label: CardLabel::J,
            },
            Card {
                label: CardLabel::K,
            },
            Card {
                label: CardLabel::A,
            },
        ]
    );
}

#[test]
fn should_sort_hands_without_high_card() {
    let full_house = Hand {
        cards: vec![
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::J,
            },
            Card {
                label: CardLabel::J,
            },
        ],
        bid: 65,
    };

    let high_card = Hand {
        cards: vec![
            Card {
                label: CardLabel::A,
            },
            Card {
                label: CardLabel::K,
            },
            Card {
                label: CardLabel::Q,
            },
            Card {
                label: CardLabel::J,
            },
            Card {
                label: CardLabel::T,
            },
        ],
        bid: 65,
    };

    let three_of_a_kind = Hand {
        cards: vec![
            Card {
                label: CardLabel::T,
            },
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::J,
            },
            Card {
                label: CardLabel::Five,
            },
        ],
        bid: 65,
    };

    let mut input = vec![
        full_house.clone(),
        high_card.clone(),
        three_of_a_kind.clone(),
    ];

    input.sort();

    assert_eq!(input, vec![high_card, three_of_a_kind, full_house]);
}

#[test]
fn should_sort_hands_with_high_cards() {
    let high_card_six = Hand {
        cards: vec![
            Card {
                label: CardLabel::Two,
            },
            Card {
                label: CardLabel::Three,
            },
            Card {
                label: CardLabel::Four,
            },
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::Six,
            },
        ],
        bid: 65,
    };

    let high_card_seven = Hand {
        cards: vec![
            Card {
                label: CardLabel::Two,
            },
            Card {
                label: CardLabel::Three,
            },
            Card {
                label: CardLabel::Four,
            },
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::Seven,
            },
        ],
        bid: 65,
    };

    let high_card_eight = Hand {
        cards: vec![
            Card {
                label: CardLabel::Two,
            },
            Card {
                label: CardLabel::Three,
            },
            Card {
                label: CardLabel::Four,
            },
            Card {
                label: CardLabel::Five,
            },
            Card {
                label: CardLabel::Eight,
            },
        ],
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
        cards: vec![
            Card {
                label: CardLabel::Three,
            },
            Card {
                label: CardLabel::Three,
            },
            Card {
                label: CardLabel::Three,
            },
            Card {
                label: CardLabel::Three,
            },
            Card {
                label: CardLabel::Two,
            },
        ],
        bid: 65,
    };

    let four_of_a_kind_2 = Hand {
        cards: vec![
            Card {
                label: CardLabel::Two,
            },
            Card {
                label: CardLabel::A,
            },
            Card {
                label: CardLabel::A,
            },
            Card {
                label: CardLabel::A,
            },
            Card {
                label: CardLabel::A,
            },
        ],
        bid: 65,
    };

    let mut hands = vec![four_of_a_kind_1.clone(), four_of_a_kind_2.clone()];

    hands.sort();

    assert_eq!(hands, vec![four_of_a_kind_2, four_of_a_kind_1]);
}

#[test]
fn should_sort_hands_with_both_have_a_full_house() {
    let full_house_1 = Hand {
        cards: vec![
            Card {
                label: CardLabel::Seven,
            },
            Card {
                label: CardLabel::Seven,
            },
            Card {
                label: CardLabel::Eight,
            },
            Card {
                label: CardLabel::Eight,
            },
            Card {
                label: CardLabel::Eight,
            },
        ],
        bid: 65,
    };

    let full_house_2 = Hand {
        cards: vec![
            Card {
                label: CardLabel::Seven,
            },
            Card {
                label: CardLabel::Seven,
            },
            Card {
                label: CardLabel::Seven,
            },
            Card {
                label: CardLabel::Eight,
            },
            Card {
                label: CardLabel::Eight,
            },
        ],
        bid: 65,
    };

    let mut hands = vec![full_house_1.clone(), full_house_2.clone()];

    hands.sort();

    assert_eq!(hands, vec![full_house_2, full_house_1]);
}
