use std::collections::{HashSet};
use std::env;
use std::path::PathBuf;

#[derive(Clone)]
struct Symbol {
    value: char,
    coordinate: Coordinate
}

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    i: usize,
    j: usize
}

fn get_grid_from_input() -> Vec<Vec<char>> {
    let file = env::current_dir().unwrap().join("day3/src/input.txt");
    let grid = create_grid_from_input(file);
    grid
}


fn main() {
    let grid = get_grid_from_input();
    part1(&grid);
    part2(&grid);
}

fn part2(grid: &Vec<Vec<char>>) {

    let mut symbol_locations: Vec<Symbol> = Vec::new();

    get_all_symbols(&grid, &mut symbol_locations);

    let star_symbols: Vec<Symbol> = symbol_locations
        .iter()
        .filter(|symbol| symbol.value == '*')
        .cloned()
        .collect();

    let gear_pairs = get_all_gear_pairs(&grid, &star_symbols);
    let sum = gear_pairs.iter().map(|(i, j)| i * j).sum::<i32>();
    println!("{}", sum);
}

fn part1(grid: &Vec<Vec<char>>) {
    let mut symbol_locations: Vec<Symbol> = Vec::new();
    let mut collision_points: HashSet<Coordinate> = HashSet::new();
    let mut part_numbers: Vec<i32> = Vec::new();

    get_all_symbols(&grid, &mut symbol_locations);
    get_all_collision_points(&grid, &symbol_locations, &mut collision_points);

    find_all_numbers_at_collision_points(&grid, &collision_points, &mut part_numbers);

    println!("{}", part_numbers.iter().sum::<i32>());
}

fn get_all_gear_pairs(grid: &Vec<Vec<char>>, symbol_locations: &Vec<Symbol>) -> Vec<(i32, i32)> {
    let mut gear_pairs: Vec<(i32, i32)> = Vec::new();
    let mut visited_coordinates: HashSet<String> = HashSet::new();

    for symbol in symbol_locations {
        let mut collisions: Vec<Option<Coordinate>> = Vec::new();
        // check north
        collisions.push(check_north(grid, symbol));
        // check north east
        collisions.push(check_north_east(grid, symbol));
        // check east
        collisions.push(check_east(grid, symbol));
        // check south east
        collisions.push(check_south_east(grid, symbol));
        // check south
        collisions.push(check_south(grid, symbol));
        // check south west
        collisions.push(check_south_west(grid, symbol));
        // check west
        collisions.push(check_west(grid, symbol));
        // check north west
        collisions.push(check_north_west(grid, symbol));

        let mut numbers: Vec<i32> = Vec::new();
        let mut temporary_visited_coordinates: HashSet<String> = HashSet::new();
        let collisions = collisions.iter().filter(|collision| collision.is_some()).cloned().collect::<Vec<Option<Coordinate>>>();

        for collision in &collisions {
            let number = find_number_from_collision(&grid, &mut temporary_visited_coordinates, &collision.as_ref().unwrap());
            if number.is_some() {
                numbers.push(number.unwrap());
            }
        }

        if numbers.len() == 2 {
            gear_pairs.push((numbers.iter().next().unwrap().clone(), numbers.iter().next_back().unwrap().clone()));
            // append temporary visited coordinates to visited coordinates
            visited_coordinates.extend(temporary_visited_coordinates);
        }
    }

    return gear_pairs;
}

fn find_all_numbers_at_collision_points(grid: &Vec<Vec<char>>, collision_points: &HashSet<Coordinate>, part_numbers: &mut Vec<i32>) {
    // map i => j
    let mut visited_coordinates: HashSet<String> = HashSet::new();

    for collision_point in collision_points {
        if visited_coordinates.contains(&*coordinate_to_string(collision_point.i, collision_point.j)) {
            continue;
        }

        let number = find_number_from_collision(&grid, &mut visited_coordinates, &collision_point);
        if number.is_some() {
            part_numbers.push(number.unwrap());
        }
    }
}

fn find_number_from_collision(grid: &&Vec<Vec<char>>, mut visited_coordinates: &mut HashSet<String>, collision_point: &&Coordinate) -> Option<i32> {
    // check if already visited
    if visited_coordinates.contains(&*coordinate_to_string(collision_point.i, collision_point.j)) {
        return None;
    }

    // add collision
    visited_coordinates.insert(coordinate_to_string(collision_point.i, collision_point.j));

    let mut number = String::new();
    let mut left_number = String::new();
    let mut right_number = String::new();


    get_right_number(grid, &mut visited_coordinates, collision_point, &mut right_number);
    get_left_number(grid, &mut visited_coordinates, collision_point, &mut left_number);

    number.push_str(&left_number.chars().rev().collect::<String>());
    number.push_str(&grid[collision_point.i][collision_point.j].to_string());
    number.push_str(&right_number);

    return Some(number.parse::<i32>().unwrap())
}

fn coordinate_to_string(i: usize, j: usize) -> String {
    return format!("{},{}", i, j);
}

fn get_left_number(grid: &Vec<Vec<char>>, visited_coordinates: &mut HashSet<String>, collision_point: &Coordinate, left_number: &mut String) {
    for j in (0..collision_point.j).rev() {
        if grid[collision_point.i][j].is_digit(10) {
            left_number.push(grid[collision_point.i][j]);
            visited_coordinates.insert(coordinate_to_string(collision_point.i, j));
        } else {
            break;
        }
    }
}

fn get_right_number(grid: &Vec<Vec<char>>, visited_coordinates: &mut HashSet<String>, collision_point: &Coordinate, right_number: &mut String) {
    for j in collision_point.j + 1..grid[collision_point.i].len() {
        if grid[collision_point.i][j].is_digit(10) {
            right_number.push(grid[collision_point.i][j]);
            visited_coordinates.insert(coordinate_to_string(collision_point.i, j));
        } else {
            break;
        }
    }
}

fn get_all_collision_points(grid: &Vec<Vec<char>>, symbol_locations: &Vec<Symbol>, collision_points: &mut HashSet<Coordinate>) {
    for symbol in symbol_locations {
        let mut collisions: HashSet<Option<Coordinate>> = HashSet::new();
        // check north
        collisions.insert(check_north(grid, symbol));
        // check north east
        collisions.insert(check_north_east(grid, symbol));
        // check east
        collisions.insert(check_east(grid, symbol));
        // check south east
        collisions.insert(check_south_east(grid, symbol));
        // check south
        collisions.insert(check_south(grid, symbol));
        // check south west
        collisions.insert(check_south_west(grid, symbol));
        // check west
        collisions.insert(check_west(grid, symbol));
        // check north west
        collisions.insert(check_north_west(grid, symbol));

        for collision in collisions {
            if collision.is_some() {
                collision_points.insert(collision.unwrap());
            }
        }
    }
}

fn check_west(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if j == 0 {
        return None;
    }

    if grid[i][j-1].is_digit(10) {
        return Some(Coordinate { i, j: j-1 });
    }

    None
}

fn check_south_west(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == grid.len() - 1 || j == 0 {
        return None;
    }

    if grid[i+1][j-1].is_digit(10) {
        return Some(Coordinate { i: i+1, j: j-1 });
    }

    None
}

fn check_south(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == grid.len() - 1 {
        return None;
    }

    if grid[i+1][j].is_digit(10) {
        return Some(Coordinate { i: i+1, j });
    }

    None
}

fn check_south_east(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == grid.len() - 1 || j == grid[i].len() - 1 {
        return None;
    }

    if grid[i+1][j+1].is_digit(10) {
        return Some(Coordinate { i: i+1, j: j+1 });
    }

    None
}

fn check_east(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if j == grid[i].len() - 1 {
        return None;
    }

    if grid[i][j+1].is_digit(10) {
        return Some(Coordinate { i, j: j+1 });
    }

    None
}

fn check_north_west(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == 0 || j == 0 {
        return None;
    }

    if grid[i-1][j-1].is_digit(10) {
        return Some(Coordinate { i: i-1, j: j-1 });
    }

    None
}

fn check_north_east(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == 0 || j == grid[i].len() - 1 {
        return None;
    }

    if grid[i-1][j+1].is_digit(10) {
        return Some(Coordinate { i: i-1, j: j+1 });
    }

    None
}

fn check_north(grid: &Vec<Vec<char>>, symbol: &Symbol) -> Option<Coordinate> {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == 0 {
        return None;
    }

    if grid[i-1][j].is_digit(10) {
        return Some(Coordinate { i: i-1, j });
    }

    None
}

fn get_all_symbols(grid: &Vec<Vec<char>>, symbol_locations: &mut Vec<Symbol>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let symbol = grid[i][j];
            if is_symbol(symbol) {
                symbol_locations.push(Symbol { value: symbol, coordinate: Coordinate { i, j } })
            }
        }
    }
}

fn is_symbol(c: char) -> bool {
    return !c.is_digit(10) && !c.is_alphabetic() && c != '.';
}

fn create_grid_from_input(file: PathBuf) -> Vec<Vec<char>> {
    let line_string = std::fs::read_to_string(file).unwrap();
    let grid: Vec<Vec<char>> = line_string
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    grid
}
