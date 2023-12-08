#[cfg(test)]
mod tests {
    use crate::{calculate_card_type, Card, Hand, HandType};

    #[test]
    fn high_card_hand_type_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("Test Hand"),
            cards: vec![
                Card { rank: 2, value: '2' },
                Card { rank: 3, value: '3' },
                Card { rank: 4, value: '4' },
                Card { rank: 5, value: '5' },
                Card { rank: 6, value: '6' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::HighCard));
    }

    #[test]
    fn one_pair_hand_type_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("Test Hand"),
            cards: vec![
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 4, value: '4' },
                Card { rank: 5, value: '5' },
                Card { rank: 6, value: '6' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::OnePair));
    }

    #[test]
    fn two_pair_hand_type_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("Test Hand"),
            cards: vec![
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 4, value: '4' },
                Card { rank: 4, value: '4' },
                Card { rank: 6, value: '6' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::TwoPair));
    }

    #[test]
    fn three_of_a_kind_hand_type_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("Test Hand"),
            cards: vec![
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 4, value: '4' },
                Card { rank: 5, value: '5' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::ThreeOfAKind));
    }

    #[test]
    fn four_of_a_kind_hand_type_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("Test Hand"),
            cards: vec![
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 5, value: '5' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::FourOfAKind));
    }

    #[test]
    fn five_of_a_kind_hand_type_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("Test Hand"),
            cards: vec![
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::FiveOfAKind));
    }

    #[test]
    fn full_house_hand_type1_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("22233"),
            cards: vec![
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 2, value: '2' },
                Card { rank: 3, value: '3' },
                Card { rank: 3, value: '3' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::FullHouse));
    }

    #[test]
    fn full_house_hand_type2_is_correctly_identified() {
        let mut hand = Hand {
            raw: String::from("11AAA"),
            cards: vec![
                Card { rank: 2, value: '1' },
                Card { rank: 2, value: '1' },
                Card { rank: 3, value: 'A' },
                Card { rank: 3, value: 'A' },
                Card { rank: 3, value: 'A' },
            ],
            bid: 10,
            win_type: None,
            card_win_types: Vec::new(),
        };

        calculate_card_type(&mut hand);

        assert_eq!(hand.win_type, Some(HandType::FullHouse));
    }

    #[cfg(test)]
    mod tests {
        use crate::calculate_hands_score;
        use super::*;

        #[test]
        fn hands_are_sorted_correctly() {
            let mut hands = vec![
                Hand {
                    raw: String::from("Hand 1"),
                    cards: vec![
                        Card { rank: 2, value: '2' },
                        Card { rank: 3, value: '3' },
                        Card { rank: 4, value: '4' },
                        Card { rank: 5, value: '5' },
                        Card { rank: 6, value: '6' },
                    ],
                    bid: 10,
                    win_type: Some(HandType::HighCard),
                    card_win_types: Vec::new(),
                },
                Hand {
                    raw: String::from("Hand 2"),
                    cards: vec![
                        Card { rank: 2, value: '2' },
                        Card { rank: 2, value: '2' },
                        Card { rank: 4, value: '4' },
                        Card { rank: 5, value: '5' },
                        Card { rank: 6, value: '6' },
                    ],
                    bid: 10,
                    win_type: Some(HandType::HighCard),
                    card_win_types: Vec::new(),
                },
            ];

            calculate_hands_score(&mut hands);

            assert_eq!(hands[0].win_type, Some(HandType::HighCard));
            assert_eq!(hands[1].win_type, Some(HandType::OnePair));
        }

        #[test]
        fn hands_with_same_type_are_sorted_by_card_rank() {
            let mut hands = vec![
                Hand {
                    raw: String::from("Hand 1"),
                    cards: vec![
                        Card { rank: 2, value: '2' },
                        Card { rank: 3, value: '3' },
                        Card { rank: 4, value: '4' },
                        Card { rank: 5, value: '5' },
                        Card { rank: 6, value: '6' },
                    ],
                    bid: 10,
                    win_type: Some(HandType::HighCard),
                    card_win_types: Vec::new(),
                },
                Hand {
                    raw: String::from("Hand 2"),
                    cards: vec![
                        Card { rank: 2, value: '2' },
                        Card { rank: 3, value: '3' },
                        Card { rank: 4, value: '4' },
                        Card { rank: 5, value: '5' },
                        Card { rank: 7, value: '7' },
                    ],
                    bid: 11,
                    win_type: Some(HandType::HighCard),
                    card_win_types: Vec::new(),
                },
            ];

            calculate_hands_score(&mut hands);

            assert_eq!(hands[0].raw, "Hand 1");
            assert_eq!(hands[1].raw, "Hand 2");
        }
    }
}