use std::{env, fs};
use std::collections::HashMap;
use num_integer::lcm;
use rayon::prelude::*;

fn main() {
    // part1();
    part2();
}

fn part2() {
    let mut lines: Vec<String> = read_file("day8/src/input.txt");
    // remove all empty lines
    lines.retain(|x| !x.is_empty());

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let instructions = lines[0].chars().collect::<Vec<char>>();

    let mut start_positions: Vec<String> = Vec::new();

    // AAA = (BBB, BBB)
    for i in 1..lines.len() {
        let line = &lines[i];
        let split: Vec<&str> = line.split(" = (").collect();
        // get the first alphabet char of the string
        let key: String = split[0].to_string();
        let val1: String = split[1].split(", ").nth(0).unwrap().to_string();
        let val2: String = split[1].split(", ").nth(1).unwrap().split(')').nth(0).unwrap().to_string();

        println!("{} = ({}, {})", key, val1, val2);

        if val1.chars().nth(2).unwrap() == 'A' {
            start_positions.push(val1.clone());
        }

        if val2.chars().nth(2).unwrap() == 'A' {
            start_positions.push(val2.clone());
        }

        if key.chars().nth(2).unwrap() == 'A' {
            start_positions.push(key.clone());
        }

        map.insert(key, (val1, val2));
    }

    // Leader Start: GNA -> DDZ; Moves: 20093
    // Leader Start: FCA -> XDZ; Moves: 12169
    // Leader Start: AAA -> ZZZ; Moves: 22357
    // Leader Start: MXA -> SRZ; Moves: 14999
    // Leader Start: VVA -> JVZ; Moves: 13301
    // Leader Start: XHA -> THZ; Moves: 17263


    // moving through the map
    let mut lcms: Vec<usize> = Vec::new();
    for start in start_positions {
        let mut current = start.to_string();
        let mut moves = 0;
        while current.chars().nth(2).unwrap() != 'Z' {
            let instruction = instructions[moves % instructions.len()];
            match instruction {
                'L' => {current = map.get(&current).unwrap().0.to_string(); moves += 1},
                'R' => {current = map.get(&current).unwrap().1.to_string(); moves += 1},
                _ => {panic!("Invalid instruction")}
            }

            // check if we are at the end
            if current.chars().nth(2).unwrap() == 'Z' {
                println!("Leader Start: {} -> {}; Moves: {}", start, current, moves);
                lcms.push(moves);
            }
        }
    }

    let result = lcms.into_iter().fold(1, |acc, x| lcm(acc, x));
    println!("LCM: {}", result);
}
fn part1() {
    let mut lines: Vec<String> = read_file("day8/src/input.txt");
    // remove all empty lines
    lines.retain(|x| !x.is_empty());

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    let instructions = lines[0].chars().collect::<Vec<char>>();

    // AAA = (BBB, BBB)
    for i in 1..lines.len() {
        let line = &lines[i];
        let split: Vec<&str> = line.split(" = (").collect();
        // get the first alphabet char of the string
        let key = split[0].to_string();
        let val1 = split[1].split(", ").nth(0).unwrap().to_string();
        let val2 = split[1].split(", ").nth(1).unwrap().chars().take_while(|&c| c.is_alphabetic()).collect();
        map.insert(key, (val1, val2));
    }

    let mut moves = 0;
    let mut current = "AAA".to_string();
    // moving through the map
    while current != "ZZZ" {
        let instruction = instructions[moves % instructions.len()];
        match instruction {
            'L' => {current = map.get(&current).unwrap().0.to_string(); moves += 1},
            'R' => {current = map.get(&current).unwrap().1.to_string(); moves += 1},
            _ => {panic!("Invalid instruction")}
        }
    }

    println!("Moves: {}", moves);

}

fn read_file(file_path: &str) -> Vec<String> {
    let file = env::current_dir().unwrap().join(file_path);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");

    return contents.split('\n').map(|s| s.trim().to_string()).collect();
}
