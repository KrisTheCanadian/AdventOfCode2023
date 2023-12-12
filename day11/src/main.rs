use std::collections::{HashMap};
use colored::Colorize;

fn main() {
    part1();
    part2();
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Galaxy {
    x: i32,
    y: i32,
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let c = grid[y][x];
            match c {
                '.' => { print!("{}", c.to_string().black().italic()); }
                '#' => { print!("{}", c.to_string().white().bold()); }
                _ => { panic!("what is this...") }
            }
        }
        println!();
    }
}

fn part2() {
    let input = include_str!("./input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    let mut galaxies: Vec<Galaxy> = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '#' {
                galaxies.push(Galaxy { y: row as i32, x: col as i32 });
            }
        }
    }

    print_grid(&grid);
    let rows_with_no_galaxy: Vec<i32> = (0..grid.len() as i32).filter(|r| !galaxies.iter().any(|g| g.y == *r)).collect();
    let cols_with_no_galaxy: Vec<i32> = (0..grid[0].len() as i32).filter(|c| !galaxies.iter().any(|g| g.x == *c)).collect();

    // calculate the distance between each galaxy and store it in a map
    let mut all_distances: HashMap<(&Galaxy, &Galaxy), i32> = HashMap::new();

    for i in 0..galaxies.len() {
        let galaxy = &galaxies[i];
        for j in i..galaxies.len() {
            let other_galaxy = &galaxies[j];
            if galaxy == other_galaxy {
                continue;
            }

            let mut distance = (galaxy.x - other_galaxy.x).abs() + (galaxy.y - other_galaxy.y).abs();

            for row in &rows_with_no_galaxy {
                // row y
                if (galaxy.y > *row && other_galaxy.y < *row) || (galaxy.y < *row && other_galaxy.y > *row) {
                    distance += (1000000) - 1;
                }
            }

            for col in &cols_with_no_galaxy {
                // col x
                if (galaxy.x > *col && other_galaxy.x < *col) || (galaxy.x < *col && other_galaxy.x > *col) {
                    distance += (1000000) - 1;
                }
            }

            all_distances.insert((galaxy, other_galaxy), distance);
        }
    }

    let mut sum: i64 = 0;
    for ((_, _), distance) in all_distances {
        sum += distance as i64;
    }


    println!("Total Distances: {}", sum);
}

fn part1() {
    let input = include_str!("./input.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    let mut galaxies: Vec<Galaxy> = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == '#' {
                galaxies.push(Galaxy { y: row as i32, x: col as i32 });
            }
        }
    }

    print_grid(&grid);
    let rows_with_no_galaxy: Vec<i32> = (0..grid.len() as i32).filter(|r| !galaxies.iter().any(|g| g.y == *r)).collect();
    let cols_with_no_galaxy: Vec<i32> = (0..grid[0].len() as i32).filter(|c| !galaxies.iter().any(|g| g.x == *c)).collect();

    // calculate the distance between each galaxy and store it in a map
    let mut all_distances: HashMap<(&Galaxy, &Galaxy), i32> = HashMap::new();

    for i in 0..galaxies.len() {
        let galaxy = &galaxies[i];
        for j in i..galaxies.len() {
            let other_galaxy = &galaxies[j];
            if galaxy == other_galaxy {
                continue;
            }

            let mut distance = (galaxy.x - other_galaxy.x).abs() + (galaxy.y - other_galaxy.y).abs();

            for row in &rows_with_no_galaxy {
                // row y
                if (galaxy.y > *row && other_galaxy.y < *row) || (galaxy.y < *row && other_galaxy.y > *row) {
                    distance += 1;
                }
            }

            for col in &cols_with_no_galaxy {
                // col x
                if (galaxy.x > *col && other_galaxy.x < *col) || (galaxy.x < *col && other_galaxy.x > *col) {
                    distance += 1;
                }
            }

            all_distances.insert((galaxy, other_galaxy), distance);
        }
    }

    let mut sum: i32 = 0;
    for ((_, _), distance) in all_distances {
        sum += distance;
    }


    println!("Total Distances: {}", sum);
}
