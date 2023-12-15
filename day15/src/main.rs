fn main() {
    part1();
    part2();
}

fn part2() {
    let input = include_str!("./input.txt");
    let strings: Vec<&str> = input.split(",").collect();
    let hashmap = process_hashmap(strings);
    print_hashmap(&hashmap);

    let mut sum = 0;
    for (k, v) in hashmap.iter().enumerate() {
        for (i, t) in v.iter().enumerate() {
            sum += ((k + 1) * (i + 1)) as i32 * t.1;
        }
    }
    println!("Part 2 - sum: {}", sum);
}

fn print_hashmap(hashmap: &Vec<Vec<(String, i32)>>) {
    for (i, v) in hashmap.iter().enumerate() {
        if v.len() > 0 {
            println!("{}: {:?}", i, v);
        }
    }
}

fn process_hashmap(strings: Vec<&str>) -> Vec<Vec<(String, i32)>> {
    let mut hashmap: Vec<Vec<(String, i32)>> = Vec::new();
    // set length of vector to 256
    for _ in 0..256 {
        hashmap.push(Vec::new());
    }

    for s in strings {
        if s.contains("=") {
            let parts: Vec<&str> = s.split("=").collect();
            let label = parts[0];
            let value: i32 = parts[1].parse().unwrap();
            let hash = hash(label);

            // check if label exists in vector
            if hashmap[hash as usize].len() > 0 {
                // look for the tuple with the label
                let mut found = false;
                for (i, v) in hashmap[hash as usize].iter().enumerate() {
                    if v.0 == label {
                        found = true;
                        hashmap[hash as usize][i].1 = value;
                        break;
                    }
                }

                if !found {
                    hashmap[hash as usize].push((label.to_string(), value));
                }

            } else {
                hashmap[hash as usize].push((label.to_string(), value));
            }

        } else if s.contains("-") {
            let parts: Vec<&str> = s.split("-").collect();
            let label = parts[0];
            let hash = hash(label);
            // check if value exists in vector
            if hashmap[hash as usize].len() > 0 {
                for (i, v) in hashmap[hash as usize].iter().enumerate() {
                    if v.0 == label {
                        hashmap[hash as usize].remove(i);
                        break;
                    }
                }
            }

        }
    }
    hashmap
}

fn part1() {
    let input = include_str!("./input.txt");
    let strings: Vec<&str> = input.split(",").collect();

    let mut sum = 0;
    for s in strings {
        sum += hash(s);
    }

    println!("Part 1 - sum: {}", sum);
}

fn hash(input: &str) -> i32 {
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
