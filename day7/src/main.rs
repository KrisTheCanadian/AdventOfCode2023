mod part1_tests;

use std::{env, fs};
use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq, Copy)]
#[derive(Debug)]
struct Card {
    rank: u8,
    value: char,
}


#[derive(PartialEq, Eq, Debug)]
struct Hand {
    raw: String,
    cards: Vec<Card>,
    bid: i32,
    win_type: Option<HandType>,
    card_win_types: Vec<Card>,
}


#[derive(Eq, Ord, PartialEq, PartialOrd, Clone)]
#[derive(Debug)]
enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

trait NextEnum {
    fn joker_upgrade(&self) -> Option<Self>
        where
            Self: Sized;
}

impl NextEnum for HandType {
    fn joker_upgrade(&self) -> Option<Self> {
        match *self {
            HandType::HighCard => Some(HandType::OnePair),
            HandType::OnePair => Some(HandType::ThreeOfAKind),
            HandType::TwoPair => Some(HandType::TwoPair), // edge cases needs additional checks
            HandType::ThreeOfAKind => Some(HandType::FourOfAKind),
            HandType::FullHouse => Some(HandType::FiveOfAKind),
            HandType::FourOfAKind => Some(HandType::FiveOfAKind),
            HandType::FiveOfAKind => Some(HandType::FiveOfAKind),
        }
    }
}

fn main() {
    let lines: Vec<String> = read_file("day7/src/input.txt");

    part1(&lines);
    part2(&lines);
}

fn part2(lines: &Vec<String>) {
    const PART: i32 = 2;
    let mut hands = parse_hands(&lines, 2);
    calculate_hands_score(&mut hands);
    calculate_final_answer(&mut hands);
}

fn part1(lines: &Vec<String>) {
    const PART: i32 = 1;
    let mut hands = parse_hands(&lines, PART);
    calculate_hands_score(&mut hands);

    calculate_final_answer(&mut hands);
}

fn calculate_final_answer(hands: &mut Vec<Hand>) {
    let mut answer = 0;
    // Print or process sorted hands
    for (index, hand) in hands.iter().enumerate() {
        println!("{} - {} {:?} - Bid: {}", index, hand.raw, hand.win_type, hand.bid);
        answer += hand.bid * (index + 1) as i32;
    }

    println!("Answer: {}", answer);
}

fn create_card_from_rank(rank: u8) -> Card {
    let value: char = match rank {
        14 => 'A',
        13 => 'K',
        12 => 'Q',
        11 => 'J',
        10 => 'T',
        _ => (rank + 48) as char
    };

    Card { rank, value }
}

fn calculate_card_type(hand: &mut Hand) {
    let mut number_per_card: HashMap<Card, i32> = HashMap::new();

    let mut sorted_cards: Vec<Card> = hand.cards.clone();
    sorted_cards.sort_by(|a, b| a.rank.cmp(&b.rank));

    for card in &sorted_cards {
        let count = number_per_card.entry(card.clone()).or_insert(0);
        *count += 1;
    }

    calculate_win_type_by_frequency(hand, &mut number_per_card);

    let jokers = sorted_cards.iter().filter(|&card| card.rank == 0).count() as i32;
    if jokers > 0 {
        // Use as_ref to get a reference, and then clone to make a new instance
        hand.win_type = hand.win_type.as_ref().unwrap().joker_upgrade();

        println!("Hand: {} - Win Type: {:?}", hand.raw, hand.win_type.clone().unwrap());

        // additional logic for two pair
        if hand.win_type == Option::from(HandType::TwoPair) {
            if jokers == 1 {
                // J XX YY -> XX YYY -> full house
                hand.win_type = Option::from(HandType::FullHouse);
            } else {
                // X JJ YY -> X YYYY -> 4 of a kind
                hand.win_type = Option::from(HandType::FourOfAKind);
            }
        }
    }
}

fn calculate_win_type_by_frequency(hand: &mut Hand, number_per_card: &mut HashMap<Card, i32>) {
    for (card, &mut frequency) in number_per_card {
        if frequency == 5 {
            if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::FiveOfAKind) {
                println!("Hand: {} - Five of a kind! from card: {:?}", hand.raw, card);
                hand.win_type = Some(HandType::FiveOfAKind);
                hand.card_win_types.push(*card);
            }
        } else if frequency == 4 {
            if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::FourOfAKind) {
                println!("Hand: {} - Four of a kind! from card: {:?}", hand.raw, card);
                hand.win_type = Some(HandType::FourOfAKind);
                hand.card_win_types.push(*card);
            }
        } else if frequency == 3 {
            // check for full house
            if hand.win_type == Some(HandType::OnePair) {
                if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::FullHouse) {
                    println!("Hand: {} - Full house! from card: {:?}", hand.raw, card);
                    hand.win_type = Some(HandType::FullHouse);
                    hand.card_win_types.push(*card);
                }
            }
            if hand.win_type == Some(HandType::TwoPair) {
                if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::FullHouse) {
                    println!("Hand: {} - Full house! from card: {:?}", hand.raw, card);
                    hand.win_type = Some(HandType::FullHouse);
                    hand.card_win_types.push(*card);
                }
            } else if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::ThreeOfAKind) {
                println!("Hand: {} - Three of a kind! from card: {:?}", hand.raw, card);
                hand.win_type = Some(HandType::ThreeOfAKind);
                hand.card_win_types.push(*card);
            }
        } else if frequency == 2 {
            if hand.win_type == Some(HandType::ThreeOfAKind) {
                if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::FullHouse) {
                    println!("Hand: {} - Full house! from card: {:?}", hand.raw, card);
                    hand.win_type = Some(HandType::FullHouse);
                    hand.card_win_types.push(*card);
                }
            }
            if hand.win_type == Some(HandType::OnePair) {
                if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::TwoPair) {
                    println!("Hand: {} - Two pair! from card: {:?}", hand.raw, card);
                    hand.win_type = Some(HandType::TwoPair);
                    hand.card_win_types.push(*card);
                }
            } else if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::OnePair) {
                println!("Hand: {} - One pair! from card: {:?}", hand.raw, card);
                hand.win_type = Some(HandType::OnePair);
                hand.card_win_types.push(*card);
            }
        } else if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::HighCard) {
            if hand.win_type == Some(HandType::ThreeOfAKind) {
                if hand.win_type.as_ref().map_or(true, |win_type| *win_type <= HandType::FullHouse) {
                    println!("Hand: {} - Full house! from card: {:?}", hand.raw, card);
                    hand.win_type = Some(HandType::FullHouse);
                    hand.card_win_types.push(*card);
                }
            }
            println!("Hand: {} - High card! from card: {:?}", hand.raw, card);
            hand.win_type = Some(HandType::HighCard);
            hand.card_win_types.push(*card);
        }
    }
}

fn calculate_hands_score(hands: &mut Vec<Hand>) {
    for hand in hands.iter_mut() {
        calculate_card_type(hand);
    }

    // sort the hands here by hand type
    hands.sort_by(|a, b| {
        let type_order_a = a.win_type.as_ref().unwrap_or(&HandType::HighCard);
        let type_order_b = b.win_type.as_ref().unwrap_or(&HandType::HighCard);

        // sort by type first
        // then sort by first card type in the vector
        // then the first card

        type_order_a.cmp(&type_order_b)
            .then_with(|| a.cards.iter().nth(0).unwrap().rank.cmp(&b.cards.iter().nth(0).unwrap().rank)) // sort by first card
            .then_with(|| a.cards.iter().nth(1).unwrap().rank.cmp(&b.cards.iter().nth(1).unwrap().rank)) // sort by second card
            .then_with(|| a.cards.iter().nth(2).unwrap().rank.cmp(&b.cards.iter().nth(2).unwrap().rank)) // sort by third card
            .then_with(|| a.cards.iter().nth(3).unwrap().rank.cmp(&b.cards.iter().nth(3).unwrap().rank)) // sort by fourth card
            .then_with(|| a.cards.iter().nth(4).unwrap().rank.cmp(&b.cards.iter().nth(4).unwrap().rank)) // sort by fifth card
    });
}

fn create_card_from_char(c: char, part: i32) -> Card {
    let rank: u8 = match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if part == 2 { 0 } else { 11 },
        'T' => 10,
        _ => c.to_digit(10).unwrap() as u8
    };

    Card { rank, value: c }
}

fn parse_hands(lines: &Vec<String>, part: i32) -> Vec<Hand> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in lines {
        let left = line.split(' ').nth(0).unwrap();
        let right = line.split(' ').nth(1).unwrap();

        let mut cards: Vec<Card> = Vec::new();
        for c in left.chars() {
            cards.push(create_card_from_char(c, part));
        }

        let bid: i32 = right.parse::<i32>().unwrap();

        hands.push(Hand { raw: left.parse().unwrap(), cards, bid, win_type: None, card_win_types: Vec::new() });
    }

    hands
}

fn read_file(file_path: &str) -> Vec<String> {
    let file = env::current_dir().unwrap().join(file_path);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    return contents.split('\n').map(|s| s.trim().to_string()).collect();
}
