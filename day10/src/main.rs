use std::{env, fs};

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    part1();
}

fn part1() {
    let mut map = read_input("day10/src/input.txt");
    remove_all_sevens(&mut map);
    let max = breadth_first_search(&mut map);
    print_map(&map);
    println!("Max: {}", max);
}

fn remove_all_sevens(map: &mut Vec<Vec<char>>) {
// replace all 7 with <
    for row in map {
        for c in row {
            if *c == '7' {
                *c = '<';
            }
        }
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn breadth_first_search(map: &mut Vec<Vec<char>>) -> i32 {
    let mut queue: Vec<(usize, usize, Vec<Direction>, usize)> = Vec::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut max = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (x, y);
            }
        }
    }
    queue.push((start.0, start.1, vec![], 0));

    // mark the steps on the map
    while queue.len() > 0 {
        let (x, y, path, steps) = queue.remove(0);

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.push((x, y));
        let c = map[y][x];
        let mut directions: Vec<Direction> = Vec::new();

        if c == '.' {
            continue;
        }

        println!("x: {}, y: {}, char: {}, steps: {}", x, y, c, steps);

        if c == 'S' {
            // check neighbours
            // north
            if y > 0 {
                let neighbour = map[y - 1][x];
                if neighbour == '|' {
                    directions.push(Direction::North);
                }
            }
            // south
            if y < map.len() - 1 {
                let neighbour = map[y + 1][x];
                if neighbour == '|' {
                    directions.push(Direction::South);
                }
            }
            // west
            if x > 0 {
                let neighbour = map[y][x - 1];
                if neighbour == '-' {
                    directions.push(Direction::West);
                }
            }
            // east
            if x < map[y].len() - 1 {
                let neighbour = map[y][x + 1];
                if neighbour == '-' || neighbour == 'J' {
                    directions.push(Direction::East);
                }
            }
            map[y][x] = (steps % 10).to_string().chars().next().unwrap();
        } else {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if steps < digit as usize {
                    map[y][x] = (steps % 10).to_string().chars().next().unwrap();
                    println!("Replaced: {} -> {}", c, map[y][x]);
                }
            } else {
                map[y][x] = (steps % 10).to_string().chars().next().unwrap();
                println!("Replaced: {} -> {}", c, map[y][x]);
            }
            directions = if let Some(d) = get_direction(c) {
                d
            } else {
                continue;
            };
        }

        if steps > max {
            max = steps;
        }

        for direction in directions {
            let mut new_path = path.clone();
            new_path.push(direction.clone());
            match direction {
                Direction::North => queue.push((x, y - 1, new_path, steps + 1)),
                Direction::South => queue.push((x, y + 1, new_path, steps + 1)),
                Direction::East => queue.push((x + 1, y, new_path, steps + 1)),
                Direction::West => queue.push((x - 1, y, new_path, steps + 1)),
            }
        }
    }
    return max as i32;
}

fn read_input(file_path: &str) -> Vec<Vec<char>> {
    let file = env::current_dir().unwrap().join(file_path);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    let input = contents.trim();
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    return map;
}


fn get_direction(c: char) -> Option<Vec<Direction>> {
    match c {
        '|' => return Some(vec![Direction::North, Direction::South]),
        '-' => return Some(vec![Direction::East, Direction::West]),
        'L' => return Some(vec![Direction::North, Direction::East]),
        'J' => return Some(vec![Direction::North, Direction::West]),
        '<' => return Some(vec![Direction::South, Direction::West]),
        'F' => return Some(vec![Direction::South, Direction::East]),
        '.' => return None,
        'S' => return None,
        _ => if c.is_digit(10) {
            return None;
        } else {
            panic!("Unknown character: {}", c);
        }
    }
}
