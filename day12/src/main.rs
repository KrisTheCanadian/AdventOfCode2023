use rayon::prelude::*;

fn main() {
    part1();
    part2();
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut springs_and_numbers: Vec<(Vec<char>, Vec<i32>)> = Vec::new();

    read_springs_and_numbers(input, &mut springs_and_numbers);

    // groups are always separated by an operational spring '.'
    // # -> damaged spring
    // . -> operational spring
    // ? -> unknown spring

    // To unfold the records, on each row, replace the list of spring conditions with five copies of itself (separated by ?)
    // and replace the list of contiguous groups of damaged springs with five copies of itself
    let mut expanded_springs_and_numbers: Vec<(Vec<char>, Vec<i32>)> = Vec::new();

    for (springs, numbers) in &springs_and_numbers {
        let expanded_springs = expand_springs(&springs, 5);
        let expanded_numbers = expand_numbers(&numbers, 5);

        expanded_springs_and_numbers.push((expanded_springs, expanded_numbers));
    }

    let mut sum = 0;
    bruteforce(&mut expanded_springs_and_numbers, &mut sum);

    println!("Part 2: {}", sum);
}

fn part1() {
    let input = include_str!("./input.txt");
    let mut springs_and_numbers: Vec<(Vec<char>, Vec<i32>)> = Vec::new();

    read_springs_and_numbers(input, &mut springs_and_numbers);
    let mut sum = 0;
    bruteforce(&mut springs_and_numbers, &mut sum);

    println!("Part 1: {}", sum);
}

fn bruteforce(springs_and_numbers: &mut Vec<(Vec<char>, Vec<i32>)>, sum: &mut usize) {
    *sum = springs_and_numbers.par_iter()
        .map(|(springs, numbers)| {
            println!("Processing: {}", springs.iter().collect::<String>());
            let mut current_perm = Vec::new();
            generate_and_count_permutations(springs, numbers, 0, &mut current_perm)
        })
        .sum();
}

fn is_valid_permutation(perm: &[char], current_numbers: &[i32], springs: &[char]) -> bool {
    let mut groups: Vec<i32> = Vec::new();
    let mut group_size = 0;
    for c in perm {
        match *c {
            '#' => { group_size += 1; }
            '.' => {
                if group_size > 0 {
                    groups.push(group_size);
                    group_size = 0;
                }
            }
            _ => {}
        }

        if groups.len() > current_numbers.len() {
            return false;
        }
    }

    if group_size > 0 {
        groups.push(group_size);
    }

    if groups.len() != current_numbers.len() {
        return false;
    }

    for i in 0..groups.len() {
        if groups[i] != current_numbers[i] {
            return false;
        }
    }

    for i in 0..springs.len() {
        if springs[i] != '?' && springs[i] != perm[i] {
            return false;
        }
    }

    true
}

fn generate_and_count_permutations(springs: &[char], groups: &[i32], index: usize, current_perm: &mut Vec<char>) -> usize {

    // count the number of '#' in the current permutation
    let damaged_springs = springs.iter().filter(|c| **c == '#').count();
    if damaged_springs > groups.iter().sum::<i32>() as usize {
        return 0;
    }

    if index == springs.len() {
        return if is_valid_permutation(current_perm, groups, springs) { 1 } else { 0 };
    }

    let mut count = 0;

    match springs[index] {
        '?' => {
            // Replace '?' with '#'
            current_perm.push('#');
            count += generate_and_count_permutations(springs, groups, index + 1, current_perm);
            current_perm.pop();

            // Replace '?' with '.'
            current_perm.push('.');
            count += generate_and_count_permutations(springs, groups, index + 1, current_perm);
            current_perm.pop();
        },
        _ => {
            // Keep the current character
            current_perm.push(springs[index]);
            count += generate_and_count_permutations(springs, groups, index + 1, current_perm);
            current_perm.pop();
        }
    }

    count
}

fn expand_springs(springs: &[char], times: usize) -> Vec<char> {
    let mut expanded = Vec::new();
    for _ in 0..times {
        expanded.extend(springs.iter());
        if times > 1 {
            expanded.push('?');
        }
    }
    expanded.pop();
    expanded
}

fn expand_numbers(numbers: &[i32], times: usize) -> Vec<i32> {
    let mut expanded = Vec::new();
    for _ in 0..times {
        expanded.extend(numbers.iter());
    }
    expanded
}

fn read_springs_and_numbers(input: &str, springs_and_numbers: &mut Vec<(Vec<char>, Vec<i32>)>) {
    for line in input.lines() {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();

        let chars_part = parts[0];
        let numbers_part = parts[1];

        // Convert the character part into a Vec<char>
        let chars: Vec<char> = chars_part.chars().collect();

        // Convert the number part into a Vec<i32>
        let numbers: Vec<i32> = numbers_part.split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        springs_and_numbers.push((chars, numbers));
    }
}
