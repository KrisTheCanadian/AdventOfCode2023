use std::{env, fs};

#[derive(PartialEq)]
enum Colors {
    Red = 0,
    Blue = 1,
    Green = 2,
}

struct Cube {
    color: Colors,
    count: i32,
}

struct Game {
    id: i32,
    subsets: Vec<Vec<Cube>>,
}

struct Bag {
    cubes: Vec<Cube>,
}

fn main() {
    challenge_one();
    challenge_two();
}

fn challenge_one() {
    let file = env::current_dir().unwrap().join("day2/src/input.txt");
    let given_bag = Bag {
        cubes: vec![
            Cube {color: Colors::Red, count: 12},
            Cube {color: Colors::Green, count: 13},
            Cube {color: Colors::Blue, count: 14}
        ]
    };
    let games: Vec<Game> = get_games(file.to_str().unwrap());
    let possible_games = get_possible_games(&games, &given_bag);

    let sum: i32 = possible_games.iter().map(|g| g.id).sum();
    println!("{}", sum);
}

fn challenge_two() {
    let file = env::current_dir().unwrap().join("day2/src/input.txt");
    let games: Vec<Game> = get_games(file.to_str().unwrap());
    let bags: Vec<Bag> =  get_smallest_bag(&games);
    println!("{}", get_power(&bags))
}

fn get_power(bags: &Vec<Bag>) -> i32 {
    let mut power: i32 = 0;

    for bag in bags {
        let bag_power: i32 = bag.cubes.iter().map(|c| c.count).product();
        power += bag_power;
    }

    return power;
}

fn get_smallest_bag(games: & Vec<Game>) -> Vec<Bag> {
    let mut bags: Vec<Bag> = Vec::new();

    for game in games {
        let mut smallest_bag = Bag {
            cubes: vec![
                Cube {color: Colors::Red, count: 0},
                Cube {color: Colors::Green, count: 0},
                Cube {color: Colors::Blue, count: 0}
            ],
        };
        for subset in &game.subsets {
            for cube in subset {
                let matching_cube = smallest_bag.cubes.iter_mut().find(|c| c.color == cube.color).unwrap();
                if cube.count > matching_cube.count {
                    matching_cube.count = cube.count;
                }
            }
        }
        bags.push(smallest_bag);
    }

    return bags;
}

fn get_possible_games<'a>(games: &'a Vec<Game>, bag: &'a Bag) -> Vec<&'a Game> {
    let mut possible_games: Vec<&Game> = Vec::new();

    for game in games {
        let mut possible: bool = true;
        for subset in &game.subsets {
            let mut subset_possible: bool = false;

            for cube in subset {
                let matching_cube = bag.cubes.iter().find(|c| c.color == cube.color).unwrap();
                if cube.count > matching_cube.count {
                    subset_possible = false;
                    break;
                } else {
                    subset_possible = true;
                }
            }

            if !subset_possible {
                possible = false;
                break;
            }
        }

        if possible {
            possible_games.push(game);
        }
    }

    return possible_games;
}

fn get_games(file_path: &str) -> Vec<Game> {
    let file = fs::read_to_string(file_path).unwrap();
    let mut games: Vec<Game> = Vec::new();

    for line in file.lines() {
        games.push(Game {
            id: get_game_id(line),
            subsets: get_game_subsets(line),
        });
    }

    return games;
}

fn get_game_id(line: &str) -> i32 {
    let game_string = line.split(":").nth(0).unwrap();
    return game_string.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
}

fn get_game_subsets(line: &str) -> Vec<Vec<Cube>> {
    let game_string = line.splitn(2, ':').nth(1).unwrap().trim();
    let subset_strings: Vec<String> = game_string.split(';').map(|s| String::from(s.trim().to_lowercase())).collect();

    return subset_strings.iter().map(|s| parse_subset_string(s)).collect();
}

fn parse_subset_string(subset_line: &str) -> Vec<Cube> {
    let mut subset: Vec<Cube> = Vec::new();

    let subset_string: Vec<String> = subset_line.split(",").map(|s| String::from(s.trim())).collect();

    for cube_string in subset_string {
        let cube: Cube = parse_cube_string(cube_string);
        subset.push(cube);
    }

    return subset;
}

fn parse_cube_string(cube_string: String) -> Cube {
    let mut color: Colors = Colors::Red;

    if cube_string.contains("red") {
        color = Colors::Red;
    } else if cube_string.contains("blue") {
        color = Colors::Blue;
    } else if cube_string.contains("green") {
        color = Colors::Green;
    }

    let number_string: String = cube_string.chars().filter(|c| c.is_digit(10)).collect();

    return Cube {
        color,
        count: number_string.parse::<i32>().unwrap(),
    };
}
