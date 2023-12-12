use std::{env, fs};
use colored::Colorize;

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
    let mut map = read_input("day10/src/input.txt");
    part1(&mut map);
    part2(&mut map);
}

fn part2(mut map: &mut Vec<Vec<char>>) {
    println!("Part 2 - Calculate inside");
    print_map(&map);
    // remove all random pipes and turn them to dirt
    turn_useless_pipes_to_dirt(&mut map);
    println!("Turning useless pipes to dirt");
    print_map(&map);

    // get all coordinates of map that are digits or 'S'
    let mut coordinates: Vec<(f64, f64)> = Vec::new();
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c.is_digit(10) || c == 'S' {
                coordinates.push((x as f64, y as f64));
            }
        }
    }

    // recover the map for every non-dirty
    recover_map(&mut map);
    replace_to_unicode(&mut map);

    println!("Recovering map");
    print_map(&map);

    let tiles_inside: i32 = calculate_inside(&mut map);
    println!("Part 2 - Tiles inside: {}", tiles_inside);
    print_map(&map);
    println!();
}

fn recover_map(map: &mut &mut Vec<Vec<char>>) {
    let new_map = read_input("day10/src/input.txt");
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c = map[y][x];
            if c.is_digit(10) {
                map[y][x] = new_map[y][x];
            }
        }
    }
}

fn turn_useless_pipes_to_dirt(map: &mut Vec<Vec<char>>) {
    for row in map {
        for c in row {
            if is_a_pipe(*c) {
                *c = '.';
            }
        }
    }
}

fn part1(mut map: &mut Vec<Vec<char>>) {
    replace_to_unicode(&mut map);
    print_map(&map);
    let max = breadth_first_search(&mut map);
    println!("Part 1 - Max: {}", max);
    print_map(&map);
    println!();
}

fn calculate_inside(map: &mut Vec<Vec<char>>) -> i32 {
    let mut sum: i32 = 0;

    print_map(&map);

    boundary_mutate(map, &mut sum);
    inside_mutate(map, &mut sum);

    sum
}

fn inside_mutate(map: &mut Vec<Vec<char>>, sum: &mut i32) {
    // get each I
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let mut visited: Vec<(usize, usize)> = Vec::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'I' {
                queue.push((x, y));
            }
        }
    }

    while queue.len() > 0 {
        let (x, y) = queue.remove(0);

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.push((x, y));
        let neighbours = get_neighbours(x, y, map);
        for (n_x, n_y) in neighbours {
            if !visited.contains(&(n_x as usize, n_y as usize)) {
                visited.push((n_x as usize, n_y as usize));
                if map[n_y as usize][n_x as usize] == 'O' || map[n_y as usize][n_x as usize] == '.' {
                    map[n_y as usize][n_x as usize] = 'I';
                    *sum += 1;
                }
                queue.push((n_x as usize, n_y as usize));
            }
        }
    }

}

fn get_border_points(map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let rows = map.len();
    let cols = map[0].len();
    let mut boundary: Vec<(usize, usize)> = Vec::new();

    for x in 0..cols {
        if !is_a_pipe(map[0][x]) {
            boundary.push((x, 0));
        }
        if !is_a_pipe(map[rows - 1][x]) {
            boundary.push((x, rows - 1));
        }
    }

    for y in 0..rows {
        if !is_a_pipe(map[y][0]) {
            boundary.push((0, y));
        }
        if !is_a_pipe(map[y][cols - 1]) {
            boundary.push((cols - 1, y));
        }
    }

    boundary
}

fn boundary_mutate(map: &mut Vec<Vec<char>>, sum: &mut i32) {
    // get all boundary points which are not pipes
    let boundary: Vec<(usize, usize)> = get_border_points(map);

    let mut queue: Vec<(usize, usize)> = Vec::new();

    for (x, y) in &boundary {
        queue.push((*x, *y));
    }

    let mut visited: Vec<(usize, usize)> = Vec::new();
    for (x, y) in boundary {
        queue.push((x, y));
    }

    while queue.len() > 0 {
        let (x, y) = queue.remove(0);

        if map[y][x] == 'I' {
            map[y][x] = 'O';
            *sum -= 1;
        } else if map[y][x] == '.' {
            map[y][x] = 'O';
        }

        let neighbours = get_neighbours(x, y, map);
        for (n_x, n_y) in neighbours {
            if !visited.contains(&(n_x as usize, n_y as usize)) {
                visited.push((n_x as usize, n_y as usize));
                if map[n_y as usize][n_x as usize] == 'I' {
                    map[n_y as usize][n_x as usize] = 'O';
                    *sum -= 1;
                }
                queue.push((n_x as usize, n_y as usize));
            }
        }
    }
}

fn get_neighbours(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut neighbours: Vec<(i32, i32)> = Vec::new();
    let rows = map.len() - 1;
    let cols = map[0].len() - 1;

    let add = |x: i32, y: i32, neighbors: &mut Vec<(i32, i32)>, _rows: usize, _cols: usize| {
        let neighbour = map[y as usize][x as usize];
        match neighbour {
            'O' | 'I' | '.' => neighbors.push((x, y)),
            _ => {}
        }
    };

    // North
    if y > 0 {
        add(x as i32, y as i32 - 1, &mut neighbours, rows, cols);
    }

    // South
    if y < rows - 1 {
        add(x as i32, y as i32 + 1, &mut neighbours, rows, cols);
    }

    // West
    if x > 0 {
        add(x as i32 - 1, y as i32, &mut neighbours, rows, cols);
    }

    // East
    if x < cols - 1 {
        add(x as i32 + 1, y as i32, &mut neighbours, rows, cols);
    }

    // North-West
    if y > 0 && x > 0 {
        add(x as i32 - 1, y as i32 - 1, &mut neighbours, rows, cols);
    }

    // North-East
    if y > 0 && x < cols - 1 {
        add(x as i32 + 1, y as i32 - 1, &mut neighbours, rows, cols);
    }

    // South-West
    if y < rows - 1 && x > 0 {
        add(x as i32 - 1, y as i32 + 1, &mut neighbours, rows, cols);
    }

    // South-East
    if y < rows - 1 && x < cols - 1 {
        add(x as i32 + 1, y as i32 + 1, &mut neighbours, rows, cols);
    }

    neighbours
}

fn is_a_pipe(c: char) -> bool {
    c == '─' || c == '│' || c == '┌' || c == '┐' || c == '└' || c == '┘' || c == 'S'
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
            match c {
                'O' => print!("{}", c.to_string().italic().blue()),
                'I' => print!("{}", c.to_string().bold().red()),
                'S' => print!("{}", c.to_string().italic().yellow()),
                '.' => print!("{}", c.to_string().italic().white()),
                &_ => { print!("{}", c); },
            }
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
