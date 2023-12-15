fn main() {
    part1();
}

fn part1() {
    let input = include_str!("./input.txt");
    let mut dish: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    print_dish(&dish);

    // move all the rocks north

    // for each column
    // move O up if there's a dot above it
    let mut is_completed = false;
    while !is_completed {
        is_completed = true;
        for col in 0..dish[0].len() {
            for row in 0..dish.len() {
                while dish[row][col] == 'O' && row > 0 && dish[row - 1][col] == '.' {
                    dish[row][col] = '.';
                    dish[row - 1][col] = 'O';
                }
            }
        }

        // check all 0 to see if there's a dot above it
        for col in 0..dish[0].len() {
            for row in 0..dish.len() {
                if dish[row][col] == 'O' && row > 0 && dish[row - 1][col] == '.' {
                    is_completed = false;
                    break;
                }
            }
        }

        if is_completed {
            break;
        }
    }

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
