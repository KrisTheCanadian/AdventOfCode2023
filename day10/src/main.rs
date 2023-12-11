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
    part2();
}

fn part2() {
    let mut map = read_input("day10/src/input.txt");
    replace_to_unicode(&mut map);
    print_map(&map);
    let tiles_inside: i32 = calculate_inside(&mut map);
    println!("Part 2 - Tiles inside: {}", tiles_inside);
    print_map(&map);
    println!();
}

fn part1() {
    let mut map = read_input("day10/src/input.txt");
    replace_to_unicode(&mut map);
    print_map(&map);
    let max = breadth_first_search(&mut map);
    println!("Part 1 - Max: {}", max);
    print_map(&map);
    println!();
}

fn calculate_inside(map: &mut Vec<Vec<char>>) -> i32 {
    let mut sum: i32 = 0;
    horizontal_trace(map, &mut sum);
    vertical_trace(map, &mut sum);
    boundary_mutate(map, &mut sum);
    sum
}

fn boundary_mutate(map: &mut Vec<Vec<char>>, sum: &mut i32) {
    let rows = map.len();
    let cols = map[0].len();

    for j in 0..cols {
        check_and_mutate(map, 0, j, sum);
        check_and_mutate(map, rows - 1, j, sum);
    }

    for i in 1..rows - 1 {
        check_and_mutate(map, i, 0, sum);
        check_and_mutate(map, i, cols - 1, sum);
    }

}

fn check_and_mutate(map: &mut Vec<Vec<char>>, i: usize, j: usize, sum: &mut i32) {
    if  map[i][j] == 'I' {
        *sum -= 1;
        map[i][j] = 'O';
    }

    // Look at neighbors
    let neighbors = get_neighbors(&map, i, j);
    for neighbor in neighbors {
        let (ni, nj) = neighbor;
        if map[ni][nj] == 'I' {
            *sum -= 1;
            map[ni][nj] = 'O';
        }
    }
}

fn get_neighbors(map: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    let directions = vec![
        (-1, 0),  // North
        (-1, 1),  // North East
        (0, 1),   // East
        (1, 1),   // South East
        (1, 0),   // South
        (1, -1),  // South West
        (0, -1),  // West
        (-1, -1), // North West
    ];

    for dir in directions {
        let ni = (i as isize + dir.0) as usize;
        let nj = (j as isize + dir.1) as usize;

        if ni < map.len() && nj < map[0].len() {
            neighbors.push((ni, nj));
        }
    }

    neighbors
}


fn is_a_pipe(c: char) -> bool {
    c == '─' || c == '│' || c == '┌' || c == '┐' || c == '└' || c == '┘'
}


fn vertical_trace(map: &mut Vec<Vec<char>>, sum: &mut i32) -> Vec<(usize, usize)> {
    let mut collisions: Vec<(usize, usize)> = Vec::new();
    for x in 0..map[0].len() {
        let mut ranges: Vec<(i32, i32)> = Vec::new();
        let mut start: i32 = -1;
        let mut end: i32 = -1;

        let mut y = 0;
        while y < map.len() {
            let c = map[y][x];

            if c.is_digit(10) || is_a_pipe(c) {
                if start != -1 {
                    end = y as i32;
                    ranges.push((start, end));
                    start = -1;
                    end = -1;
                }
                // continue until next digit
                for z in y + 1..map.len() {
                    let c = map[z][x];
                    if c.is_digit(10) || is_a_pipe(c) {
                        continue;
                    } else {
                        y = z;
                        break;
                    }
                }

                if start == -1 {
                    start = y as i32;
                }
            }
            y += 1;
        }

        for range in &ranges {
            for y in range.0..range.1 {
                if map[y as usize][x] == '.' || map[y as usize][x] == 'I' {
                    collisions.push((y as usize, x));
                    if map[y as usize][x] == '.' {
                        map[y as usize][x] = 'I';
                        *sum += 1;
                    }
                }
            }
        }

        for y in 0..map.len() {
            if map[y][x] == '.' {
                map[y][x] = 'O';
            }
        }
    }
    collisions
}


fn horizontal_trace(map: &mut Vec<Vec<char>>, sum: &mut i32) -> Vec<(usize, usize)> {
    let mut collisions: Vec<(usize, usize)> = Vec::new();
    for y in 0..map.len() {
        let mut ranges: Vec<(i32, i32)> = Vec::new();
        let mut start: i32 = -1;
        let mut end: i32 = -1;

        let mut x = 0;
        while x < map[y].len() {
            let c = map[y][x];

            if c.is_digit(10) || is_a_pipe(c) {
                if start != -1 {
                    end = x as i32;
                    ranges.push((start, end));
                    start = -1;
                    end = -1;
                }
                // continue until next digit
                for z in x + 1..map[y].len() {
                    let c = map[y][z];
                    if c.is_digit(10) || is_a_pipe(c) {
                        continue;
                    } else {
                        x = z;
                        break;
                    }
                }


                if start == -1 {
                    start = x as i32;
                }
            }
            x += 1;
        }
        for range in &ranges {
            for x in range.0..range.1 {
                if map[y][x as usize] == '.' || map[y][x as usize] == 'I' {
                    collisions.push((y, x as usize));
                    map[y][x as usize] = 'I';
                    *sum += 1;
                }
            }
        }

        for x in 0..map[y].len() {
            if map[y][x] == '.' {
                map[y][x] = 'O';
            }
        }
    }
    collisions
}

fn replace_to_unicode(map: &mut Vec<Vec<char>>) {
// replace all 7 with <
    for row in map {
        for c in row {
            match c {
                '-' => *c = '─',
                '|' => *c = '│',
                'F' => *c = '┌',
                '7' => *c = '┐',
                'L' => *c = '└',
                'J' => *c = '┘',
                _ => continue,
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

        if c == 'S' {
            // check neighbours
            // north
            if y > 0 {
                let neighbour = map[y - 1][x];
                match neighbour {
                    '│' | '┌' | '┐' => directions.push(Direction::North),
                    _ => {},
                }
            }
            // south
            if y < map.len() - 1 {
                let neighbour = map[y + 1][x];
                match neighbour {
                    '│' | '└' | '┘' => directions.push(Direction::South),
                    _ => {},
                }
            }
            // west
            if x > 0 {
                let neighbour = map[y][x - 1];
                match neighbour {
                    '─' | '┌' | '└' => directions.push(Direction::West),
                    _ => {},
                }
            }
            // east
            if x < map[y].len() - 1 {
                let neighbour = map[y][x + 1];
                match neighbour {
                    '─' | '┐' | '┘' => directions.push(Direction::East),
                    _ => {},
                }
            }
            map[y][x] = (steps % 10).to_string().chars().next().unwrap();
        } else {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if steps < digit as usize {
                    map[y][x] = (steps % 10).to_string().chars().next().unwrap();
                }
            } else {
                map[y][x] = (steps % 10).to_string().chars().next().unwrap();
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
        '│' => return Some(vec![Direction::North, Direction::South]),
        '─' => return Some(vec![Direction::East, Direction::West]),
        '└' => return Some(vec![Direction::North, Direction::East]),
        '┘' => return Some(vec![Direction::North, Direction::West]),
        '┐' => return Some(vec![Direction::South, Direction::West]),
        '┌' => return Some(vec![Direction::South, Direction::East]),
        '.' => return None,
        'S' => return None,
        _ => if c.is_digit(10) {
            return None;
        } else {
            panic!("Unknown character: {}", c);
        }
    }
}
