use std::{env, fs};


struct Card {
    number: i32,
    winnings: Vec<i32>,
    actual: Vec<i32>,
    actual_winnings: Vec<i32>,
    total: i32,
}

fn main() {
    part1();
}

fn part1() {
    let cards: Vec<Card> = get_cards("day4/src/input.txt");
    // get the sum of all the totals
    let total: i32 = cards.iter().map(|x| x.total).sum();
    println!("Total: {}", total);
}

fn get_cards(file_path: &str) -> Vec<Card> {
    let file = env::current_dir().unwrap().join(file_path);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut cards: Vec<Card> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<String> = line.splitn(2, "Card ").map(String::from).collect();
        let card_split: Vec<String> = parts[1].splitn(2, ":").map(String::from).collect();
        let card_content = card_split[1].to_string();
        let number = card_split[0].trim().chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap();

        let numbers = card_content.split(" | ").map(String::from).collect::<Vec<String>>();
        let winnings = numbers[0].split(" ").map(String::from).collect::<Vec<String>>();
        let actual = numbers[1].split(" ").map(String::from).collect::<Vec<String>>();

        // remove all the empty strings
        let winnings = winnings.into_iter().filter(|x| !x.is_empty()).collect::<Vec<String>>();
        let actual = actual.into_iter().filter(|x| !x.is_empty()).collect::<Vec<String>>();

        // convert all string numbers to i32
        let winnings = winnings.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let actual = actual.iter().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        let (total, actual_winnings) = calculate_points(&winnings, &actual);

        cards.push(Card {
            number,
            winnings,
            actual,
            actual_winnings,
            total,
        });
    }


    return cards;
}

fn calculate_points(winnings: &Vec<i32>, actual: &Vec<i32>) -> (i32, Vec<i32>) {
    // get all numbers that are in both winnings and actual
    let mut actual_winnings: Vec<i32> = Vec::new();
    for number in winnings {
        if actual.contains(&number) {
            actual_winnings.push(*number);
        }
    }

    let mut total_wins = actual_winnings.len() as i32;

    if total_wins == 0 {
        return (0, actual_winnings);
    }

    if total_wins == 1 {
        return (1, actual_winnings);
    }

    if total_wins > 1 {
        total_wins = 1;
        for _ in 1..actual_winnings.len() {
            total_wins *= 2;
        }
    }

    return (total_wins, actual_winnings);
}
