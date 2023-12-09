use std::{env, fs};
use std::collections::VecDeque;

fn main() {
    part1();
}

fn part1() {
    let inputs: VecDeque<VecDeque<i32>> = read_file("day9/src/input.txt");
    let mut answer = 0;
    for input in inputs {
        let mut setup: VecDeque<VecDeque<i32>> = generate_setup(&input);
        generate_extrapolation(&mut setup);
        // add put the first row last number to the answer
        answer += setup[0][setup[0].len() - 1];
    }
    println!("Answer: {}", answer);
}

fn generate_extrapolation(rows: &mut VecDeque<VecDeque<i32>>) {
    // add extra number to the end of each row
    let mut previous_row_last_number = 0;
    for row in rows.iter_mut().rev() {
        let last_number = row[row.len() - 1];
        if last_number == 0 {
            row.push_back(last_number);
            continue;
        }

        let new_number = previous_row_last_number + last_number;
        previous_row_last_number = new_number;
        // row push back last number
        row.push_back(new_number);
    }
}

// 0 3 6 9 12 15
// 0 3 6 9 12 15
//  3 3 3 3  3
//   0 0 0 0
fn generate_setup(inputs: &VecDeque<i32>) -> VecDeque<VecDeque<i32>> {
    let mut rows: VecDeque<VecDeque<i32>> = VecDeque::new();
    rows.push_back(inputs.clone());
    let mut current_setup = 0;
    // calculating changes
    loop {
        let mut diffs: VecDeque<i32> = VecDeque::new();
        let current_vec = &rows[current_setup];

        for index in 0..current_vec.len() {
            if index == current_vec.len() - 1 {
                continue;
            }

            let diff: i32 = current_vec[index + 1] - current_vec[index];
            diffs.push_back(diff.clone());
        }

        current_setup += 1;
        rows.push_back(diffs.clone());

        if diffs.iter().all(|&x| x == 0) {
            break;
        }
    }
    rows
}

fn read_file(file_path: &str) -> VecDeque<VecDeque<i32>> {
    let file = env::current_dir().unwrap().join(file_path);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines = VecDeque::from(contents.split('\n').map(|s| s.trim().to_string()).collect::<Vec<String>>());
    let inputs: VecDeque<VecDeque<i32>> = lines.iter().map(|line| {
        let numbers: VecDeque<i32> = line.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        numbers
    }).collect();
    inputs
}

