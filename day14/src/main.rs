use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part2() {
    let input = include_str!("./input.txt");
    let mut dish: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    print_dish(&dish);

    for i in 0..1000 {
        move_rocks_north(&mut dish);
        move_rocks_west(&mut dish);
        move_rocks_south(&mut dish);
        move_rocks_east(&mut dish);
    }


    print_dish(&dish);
    println!("Part2 - load: {}", calculate_load(&mut dish));
}

fn calculate_load(dish: &mut Vec<Vec<char>>) -> usize {
    let mut load = 0;
    for row in 0..dish.len() {
        for col in 0..dish[0].len() {
            if dish[row][col] == 'O' {
                load += 1 * (dish.len() - row);
            }
        }
    }
    load
}

fn part1() {
    let input = include_str!("./input.txt");
    let mut dish: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    print_dish(&dish);

    move_rocks_north(&mut dish);

    // calculate load
    let mut load = 0;
    for row in 0..dish.len() {
        for col in 0..dish[0].len() {
            if dish[row][col] == 'O' {
                load += 1 * (dish.len() - row);
            }
        }
    }

    print_dish(&dish);
    println!("Part1 - load: {}", load);
}

fn move_rocks_east(dish: &mut Vec<Vec<char>>) {
    let rows = dish.len();
    let cols = dish[0].len();

    let mut is_completed = false;
    while !is_completed {
        is_completed = true;
        for row in 0..rows {
            for col in (0..cols - 1).rev() { // iterate in reverse
                if dish[row][col] == 'O' && dish[row][col + 1] == '.' {
                    dish[row][col] = '.';
                    dish[row][col + 1] = 'O';
                    is_completed = false;
                }
            }
        }
    }
}


fn move_rocks_south(dish: &mut Vec<Vec<char>>) {
    let rows = dish.len();
    let cols = dish[0].len();

    let mut is_completed = false;
    while !is_completed {
        is_completed = true;
        for col in 0..cols {
            for row in (0..rows - 1).rev() { // iterate in reverse
                if dish[row][col] == 'O' && dish[row + 1][col] == '.' {
                    dish[row][col] = '.';
                    dish[row + 1][col] = 'O';
                    is_completed = false;
                }
            }
        }
    }
}


fn move_rocks_west(dish: &mut Vec<Vec<char>>) {
    let rows = dish.len();
    let cols = dish[0].len();

    let mut is_completed = false;
    while !is_completed {
        is_completed = true;
        for row in 0..rows {
            for col in 1..cols { // start from second column
                if dish[row][col] == 'O' && dish[row][col - 1] == '.' {
                    dish[row][col] = '.';
                    dish[row][col - 1] = 'O';
                    is_completed = false;
                }
            }
        }
    }
}


fn move_rocks_north(dish: &mut Vec<Vec<char>>) {
    let rows = dish.len();
    let cols = dish[0].len();

    let mut is_completed = false;
    while !is_completed {
        is_completed = true;
        for col in 0..cols {
            for row in 1..rows { // start from the second row
                if dish[row][col] == 'O' && dish[row - 1][col] == '.' {
                    dish[row][col] = '.';
                    dish[row - 1][col] = 'O';
                    is_completed = false;
                }
            }
        }
    }
}

fn print_dish(dish: &Vec<Vec<char>>) {
    println!();
    for row in dish {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}
