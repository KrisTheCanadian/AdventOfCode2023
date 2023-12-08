use std::{env, fs};
use std::collections::HashMap;

fn main() {
    part1();
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
