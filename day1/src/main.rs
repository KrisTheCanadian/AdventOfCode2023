use std::{env, fs};

fn main() {
    challenge_one();
    challenge_two();
}

fn challenge_two() {
    let file_path = env::current_dir().unwrap().join("day1/src/input2.txt");
    let mut input_lines: Vec<String> = read_input(file_path.to_str().unwrap());
    let mut parsed_lines: Vec<String> = Vec::new();

    for line in input_lines {
        let parsed_line: String = line
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine");
        parsed_lines.push(parsed_line);
    }

    let sum = calculate_sum(&parsed_lines);
    println!("{}", sum);
}

fn challenge_one() {
    let file_path = env::current_dir().unwrap().join("day1/src/input.txt");
    let input_lines: Vec<String> = read_input(file_path.to_str().unwrap());

    let sum: u32 = calculate_sum(&input_lines);
    println!("{}", sum);
}

fn calculate_sum(input_lines: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;

    for line in input_lines {
        let numbers: String = line.chars().filter(|c| c.is_digit(10)).collect();
        let first = numbers.chars().nth(0).unwrap().to_digit(10).unwrap();
        let last = numbers.chars().last().unwrap().to_digit(10).unwrap();

        sum += (first * 10) + last;
    }

    return sum;
}

fn read_input(path: &str) -> Vec<String> {
    return fs::read_to_string(path).unwrap().lines().map(String::from).collect();
}
