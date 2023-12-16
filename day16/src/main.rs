use std::collections::HashMap;

fn main() {
    part1();
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
#[derive(Clone)]
struct Beam {
    direction: Direction,
    position: (usize, usize),
}

fn part1() {
    let matrix = read_input();
    print_matrix(&matrix);
    // top left beam going right
    let beam = Beam {
        direction: Direction::Right,
        position: (0, 0),
    };
    let mut energized_tiles: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
    trace_beams(&matrix, beam, &mut energized_tiles);

    println!("Part 1: {}", energized_tiles.len());
}

fn trace_beams(matrix: &Vec<Vec<char>>, initial_beam: Beam, visited: &mut HashMap<(usize, usize), Vec<Direction>>) {
    let mut stack = vec![initial_beam];

    while let Some(beam) = stack.pop() {
        println!("Processing Beam: {:?}", beam);
        println!("Visited Length: {:?}", visited.len());

        // Same bounds and visited checks as before
        if beam.position.0 >= matrix.len() || beam.position.1 >= matrix[0].len() {
            continue;
        }

        if let Some(directions) = visited.get(&(beam.position.0, beam.position.1)) {
            if directions.contains(&beam.direction) {
                continue;
            }
        }

        let current = matrix[beam.position.0][beam.position.1];

        if visited.contains_key(&(beam.position.0, beam.position.1)) {
            let directions = visited.get(&(beam.position.0, beam.position.1)).unwrap();
            if directions.contains(&beam.direction) {
                return;
            } else {
                let mut directions = directions.clone();
                directions.push(beam.direction.clone());
                visited.insert((beam.position.0, beam.position.1), directions);
            }
        } else {
            visited.insert((beam.position.0, beam.position.1), vec![beam.direction.clone()]);
        }

        if is_mirror(current) {
            let new_beams = get_new_position_mirror(&beam, current, matrix[0].len(), matrix.len());
            for new_beam in new_beams {
                stack.push(new_beam);
            }
        } else {
            if let Some(new_beam) = get_new_position_empty_space(&beam, matrix[0].len(), matrix.len()) {
                stack.push(new_beam);
            }
        }
    }
}

fn is_mirror(c: char) -> bool {
    match c {
        '/' | '\\' | '-' | '|' => true,
        _ => false,
    }
}

fn get_new_position_mirror(beam: &Beam, current: char, col_max: usize, row_max: usize) -> Vec<Beam> {
    let (row, col) = beam.position;

    match current {
        '\\' => {
            // if beam is going up and hits a backslash, it will go left
            if beam.direction == Direction::Up {
                if col == 0 {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Left,
                    position: (row, col - 1),
                }];
            }

            // if beam is going down and hits a backslash, it will go right
            if beam.direction == Direction::Down {
                if col == col_max {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Right,
                    position: (row, col + 1),
                }];
            }

            // if beam is going left and hits a backslash, it will go up
            if beam.direction == Direction::Left {
                if row == 0 {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Up,
                    position: (row - 1, col),
                }];
            }

            // if beam is going right and hits a backslash, it will go down
            if beam.direction == Direction::Right {
                if row == row_max {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Down,
                    position: (row + 1, col),
                }];
            }

            return Vec::new();
        },
        '/' => {
            // if beam is going up and hits a forward slash, it will go right
            if beam.direction == Direction::Up {
                if col == col_max {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Right,
                    position: (row, col + 1),
                }]
            }

            // if beam is going down and hits a forward slash, it will go left
            if beam.direction == Direction::Down {
                if col == 0 {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Left,
                    position: (row, col - 1),
                }];
            }

            // if beam is going left and hits a forward slash, it will go down
            if beam.direction == Direction::Left {
                if row == row_max {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Down,
                    position: (row + 1, col),
                }];
            }

            // if beam is going right and hits a forward slash, it will go up
            if beam.direction == Direction::Right {
                if row == 0 {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Up,
                    position: (row - 1, col),
                }];
            }

            return Vec::new();
        },
        '|' => {
            if beam.direction == Direction::Up {
                if row == 0 {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Up,
                    position: (row - 1, col),
                }]
            }

            if beam.direction == Direction::Down {
                if row == row_max {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Down,
                    position: (row + 1, col),
                }]
            }

            if beam.direction == Direction::Left || beam.direction == Direction::Right {
                // beam is split into two beams, one going up and one going down
                let mut beams = Vec::new();
                if row != 0 {
                    beams.push(Beam {
                        direction: Direction::Up,
                        position: (row - 1, col),
                    });
                }
                if row != row_max {
                    beams.push(Beam {
                        direction: Direction::Down,
                        position: (row + 1, col),
                    });
                }
                return beams;
            }

            return Vec::new();
        },
        '-' => {
            if beam.direction == Direction::Left {
                if col == 0 {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Left,
                    position: (row, col - 1),
                }]
            }

            if beam.direction == Direction::Right {
                if col == col_max {
                    return Vec::new();
                }
                return vec![Beam {
                    direction: Direction::Right,
                    position: (row, col + 1),
                }]
            }

            if beam.direction == Direction::Up || beam.direction == Direction::Down {
                // beam is split into two beams, one going left and one going right
                let mut beams = Vec::new();
                if col != 0 {
                    beams.push(Beam {
                        direction: Direction::Left,
                        position: (row, col - 1),
                    });
                }
                if col != col_max {
                    beams.push(Beam {
                        direction: Direction::Right,
                        position: (row, col + 1),
                    });
                }
                return beams;
            }

            return Vec::new();
        },
        _ => {panic!("Invalid character")}
    }
}

fn get_new_position_empty_space(beam: &Beam, col_max: usize, row_max: usize) -> Option<Beam> {
    let (row, col) = beam.position;
    return match beam.direction {
        Direction::Up => {
            if row == 0 {
                return None;
            }
            Some(Beam {
                direction: Direction::Up,
                position: (row - 1, col),
            })
        },
        Direction::Down => {
            if row == row_max {
                return None;
            }
            Some(Beam {
                direction: Direction::Down,
                position: (row + 1, col),
            })
        },
        Direction::Left => {
            if col == 0 {
                return None;
            }
            Some(Beam {
                direction: Direction::Left,
                position: (row, col - 1),
            })
        },
        Direction::Right => {
            if col == col_max {
                return None;
            }
            Some(Beam {
                direction: Direction::Right,
                position: (row, col + 1),
            })
        },
    };
}

fn read_input() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    matrix
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for line in matrix {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}
