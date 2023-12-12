use itertools::{Itertools, repeat_n};

fn main() {
    // part1();
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
    for i in 0..springs_and_numbers.len() {
        let current_numbers = &springs_and_numbers[i].1;
        let number_of_required_damage_springs: i32 = springs_and_numbers[i].1.iter().sum();
        // 1, 1, 3
        // # -> 5
        // . -> 3
        // ???.###

        // calculate permutations (probably a better way to do this...)
        let mut permutations: Vec<Vec<&char>> = repeat_n(['#', '.'].iter(), springs_and_numbers[i].0.len()).multi_cartesian_product().collect();

        // remove permutations that don't match the required number of damaged springs
        permutations.retain(|p| p.iter().filter(|&c| **c == '#').count() == number_of_required_damage_springs as usize);

        permutations.retain(|p| {
            let mut sequence_matches = true;
            for j in 0..p.len() {
                if springs_and_numbers[i].0[j] == '?' {
                    continue;
                }
                if springs_and_numbers[i].0[j] != *p[j] {
                    sequence_matches = false;
                    break;
                }
            }
            sequence_matches
        });

        // calculate the groups for each permutation
        let mut permutations_groups: Vec<(&Vec<&char>, Vec<i32>)> = Vec::new();
        for p in &permutations {
            let mut groups: Vec<i32> = Vec::new();
            let mut group_size = 0;
            for c in p {
                match **c {
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
                    groups.clear();
                    break;
                }
            }

            if group_size > 0 {
                groups.push(group_size);
            }

            permutations_groups.push((p, groups));
        }

        // remove permutations that don't match the vector numbers
        permutations_groups.retain(|p| {
            let mut match_numbers = true;
            if p.1.len() != current_numbers.len() {
                match_numbers = false;
            }
            for i in 0..p.1.len() {
                if p.1[i] != current_numbers[i] {
                    match_numbers = false;
                    break;
                }
            }

            match_numbers
        });
        *sum += permutations_groups.len();
    }
}

fn expand_springs(springs: &[char], times: usize) -> Vec<char> {
    let mut expanded = Vec::new();
    for _ in 0..times {
        expanded.extend(springs.iter());
        if times > 1 {
            expanded.push('?');
        }
    }
    expanded.pop(); // Remove the last '?' added
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
