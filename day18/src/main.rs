#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    distance: i32,
    colour: String,
}

fn main() {
    part1();
}

fn part1() {
    let dig_instructions = read_input();
    // println!("{:?}", dig_instructions);

    let mut matrix = generate_matrix(&dig_instructions);
    // print_matrix(&matrix);

    let count = count_inside_tiles(&mut matrix);

    // print_matrix(&matrix);

    println!("Part 1: Count: {}", count);
}

fn count_inside_tiles(matrix: &mut Vec<Vec<char>>) -> i32 {
    let rows_max = matrix.len();
    let cols_max = matrix[0].len();

    let mut outsiders: Vec<(usize, usize)> = Vec::new();

    // mutate edges of matrix to be O
    for row in 0..rows_max {
        matrix[row][0] = 'O';
        matrix[row][cols_max - 1] = 'O';

        outsiders.push((row, 0));
        outsiders.push((row, cols_max - 1));
    }

    for col in 0..cols_max {
        matrix[0][col] = 'O';
        matrix[rows_max - 1][col] = 'O';

        outsiders.push((0, col));
        outsiders.push((rows_max - 1, col));
    }

    // mutate all neighbours of O that are . to O
    let mut mutated = true;
    while mutated {
        mutated = false;
        let mut new_outsiders: Vec<(usize, usize)> = Vec::new();
        for outsider in outsiders {
            let (row, col) = outsider;
            if row > 0 && matrix[row - 1][col] == '.' {
                matrix[row - 1][col] = 'O';
                new_outsiders.push((row - 1, col));
                mutated = true;
            }
            if row < rows_max - 1 && matrix[row + 1][col] == '.' {
                matrix[row + 1][col] = 'O';
                new_outsiders.push((row + 1, col));
                mutated = true;
            }
            if col > 0 && matrix[row][col - 1] == '.' {
                matrix[row][col - 1] = 'O';
                new_outsiders.push((row, col - 1));
                mutated = true;
            }
            if col < cols_max - 1 && matrix[row][col + 1] == '.' {
                matrix[row][col + 1] = 'O';
                new_outsiders.push((row, col + 1));
                mutated = true;
            }
        }
        outsiders = new_outsiders;
    }

    // count all remaining . and #
    let mut count = 0;
    for row in 0..rows_max {
        for col in 0..cols_max  {
            if matrix[row][col] == '.' || matrix[row][col] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

fn generate_matrix(dig_instructions: &Vec<DigInstruction>) -> Vec<Vec<char>> {
    let (min_x, max_x, min_y, max_y) = calculate_range(&dig_instructions);

    let mut matrix:Vec<Vec<char>> = vec![vec!['.'; (max_x - min_x) as usize + 3]; (max_y - min_y) as usize + 3];

    let mut x = 0 - min_x + 1;
    let mut y = 0 - min_y + 1;

    // println!("Starting Positions: x: {}, y: {}", x, y);

    for dig_instruction in dig_instructions {
        match dig_instruction.direction {
            Direction::Up => {
                for _ in 0..dig_instruction.distance {
                    matrix[y as usize][x as usize] = '#';
                    y += 1;
                }
            },
            Direction::Down => {
                for _ in 0..dig_instruction.distance {
                    matrix[y as usize][x as usize] = '#';
                    y -= 1;
                }
            },
            Direction::Left => {
                for _ in 0..dig_instruction.distance {
                    matrix[y as usize][x as usize] = '#';
                    x -= 1;
                }
            },
            Direction::Right => {
                for _ in 0..dig_instruction.distance {
                    matrix[y as usize][x as usize] = '#';
                    x += 1;
                }
            },
        }
    }

    return matrix;
}

fn calculate_range(dig_instructions: &Vec<DigInstruction>) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut max_x = 0;
    let mut min_y = i32::MAX;
    let mut max_y = 0;

    let mut x = 0;
    let mut y = 0;

    for dig_instruction in dig_instructions {
        match dig_instruction.direction {
            Direction::Up => y += dig_instruction.distance,
            Direction::Down => y -= dig_instruction.distance,
            Direction::Left => x -= dig_instruction.distance,
            Direction::Right => x += dig_instruction.distance,
        }

        if x < min_x {
            min_x = x;
        } else if x > max_x {
            max_x = x;
        }

        if y < min_y {
            min_y = y;
        } else if y > max_y {
            max_y = y;
        }
    }

    return (min_x, max_x, min_y, max_y);
}

fn read_input() -> Vec<DigInstruction> {
    include_str!("input.txt").lines().map(|line| {
        let mut parts = line.split_whitespace();
        let dir = match parts.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unknown direction"),
        };

        let dist = parts.next().unwrap().parse::<i32>().unwrap();

        let colour = parts.next().unwrap().to_string();
        let colour_start = colour.find('#').unwrap();
        let colour = &colour[colour_start + 1 .. colour_start + 7];

        return DigInstruction {
            direction: dir,
            distance: dist,
            colour: colour.to_string(),
        };
    }).collect()
}
