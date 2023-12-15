fn main() {
    part1();
}

fn part1() {
    let input = include_str!("./input.txt");
    let strings: Vec<&str> = input.split(",").collect();

    let mut sum = 0;
    for s in strings {
        sum += hash_algorithm(s);
    }

    println!("sum: {}", sum);
}

fn hash_algorithm(input: &str) -> i32 {
    let chars: Vec<char> = input.chars().collect();
    let mut current_value: i32 = 0;
    for c in chars {
        // get ascii value of c
        let ascii_value = c as u8;
        current_value += ascii_value as i32;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}
