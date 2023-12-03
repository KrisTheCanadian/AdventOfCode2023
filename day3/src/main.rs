use std::collections::{HashSet};
use std::env;
use std::path::PathBuf;

struct Symbol {
    coordinate: Coordinate
}

#[derive(Eq, Hash, PartialEq)]
struct Coordinate {
    i: usize,
    j: usize
}

fn main() {
    part1();
}

fn part1() {
    let file = env::current_dir().unwrap().join("day3/src/input.txt");

    let grid = create_grid_from_input(file);
    let mut symbol_locations: Vec<Symbol> = Vec::new();
    let mut collision_points: HashSet<Coordinate> = HashSet::new();

    let mut part_numbers: Vec<i32> = Vec::new();

    get_all_symbols(&grid, &mut symbol_locations);
    get_all_collision_points(&grid, &symbol_locations, &mut collision_points);

    find_all_numbers_at_collision_points(&grid, &collision_points, &mut part_numbers);

    println!("{}", part_numbers.iter().sum::<i32>());
}

fn find_all_numbers_at_collision_points(grid: &Vec<Vec<char>>, collision_points: &HashSet<Coordinate>, part_numbers: &mut Vec<i32>) {
    // map i => j
    let mut visited_coordinates: HashSet<String> = HashSet::new();

    for collision_point in collision_points {
        if visited_coordinates.contains(&*coordinate_to_string(collision_point.i, collision_point.j)) {
            continue;
        }

        let mut number = String::new();
        let mut left_number = String::new();
        let mut right_number = String::new();

        // add collision
        visited_coordinates.insert(coordinate_to_string(collision_point.i, collision_point.j));


        get_right_number(grid, &mut visited_coordinates, collision_point, &mut right_number);


        get_left_number(grid, &mut visited_coordinates, collision_point, &mut left_number);

        number.push_str(&left_number.chars().rev().collect::<String>());
        number.push_str(&grid[collision_point.i][collision_point.j].to_string());
        number.push_str(&right_number);

        part_numbers.push(number.parse::<i32>().unwrap());
    }
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
        // check north
        check_north(grid, symbol, collision_points);
        // check north east
        check_north_east(grid, symbol, collision_points);
        // check east
        check_east(grid, symbol, collision_points);
        // check south east
        check_south_east(grid, symbol, collision_points);
        // check south
        check_south(grid, symbol, collision_points);
        // check south west
        check_south_west(grid, symbol, collision_points);
        // check west
        check_west(grid, symbol, collision_points);
        // check north west
        check_north_west(grid, symbol, collision_points);
    }
}

fn check_west(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if j == 0 {
        return;
    }

    if grid[i][j-1].is_digit(10) {
        collision_points.insert(Coordinate { i, j: j-1 });
    }
}

fn check_south_west(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == grid.len() - 1 || j == 0 {
        return;
    }

    if grid[i+1][j-1].is_digit(10) {
        collision_points.insert(Coordinate { i: i+1, j: j-1 });
    }
}

fn check_south(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == grid.len() - 1 {
        return;
    }

    if grid[i+1][j].is_digit(10) {
        collision_points.insert(Coordinate { i: i+1, j });
    }
}

fn check_south_east(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == grid.len() - 1 || j == grid[i].len() - 1 {
        return;
    }

    if grid[i+1][j+1].is_digit(10) {
        collision_points.insert(Coordinate { i: i+1, j: j+1 });
    }
}

fn check_east(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if j == grid[i].len() - 1 {
        return;
    }

    if grid[i][j+1].is_digit(10) {
        collision_points.insert(Coordinate { i, j: j+1 });
    }
}

fn check_north_west(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == 0 || j == 0 {
        return;
    }

    if grid[i-1][j-1].is_digit(10) {
        collision_points.insert(Coordinate { i: i-1, j: j-1 });
    }
}

fn check_north_east(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == 0 || j == grid[i].len() - 1 {
        return;
    }

    if grid[i-1][j+1].is_digit(10) {
        collision_points.insert(Coordinate { i: i-1, j: j+1 });
    }
}

fn check_north(grid: &Vec<Vec<char>>, symbol: &Symbol, collision_points: &mut HashSet<Coordinate>) {
    let i = symbol.coordinate.i;
    let j = symbol.coordinate.j;

    if i == 0 {
        return;
    }

    if grid[i-1][j].is_digit(10) {
        collision_points.insert(Coordinate { i: i-1, j });
    }
}

fn get_all_symbols(grid: &Vec<Vec<char>>, symbol_locations: &mut Vec<Symbol>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let symbol = grid[i][j];
            if is_symbol(symbol) {
                symbol_locations.push(Symbol { coordinate: Coordinate { i, j } })
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
