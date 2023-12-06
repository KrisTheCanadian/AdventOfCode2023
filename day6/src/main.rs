use std::{env, fs};

struct Race {
    time: i64,
    best_distance: i64
}

fn main() {
    part1();
}

fn part1() {
    let contents: Vec<String> = read_file("day6/src/input.txt");
    let races: Vec<Race> = parse_races(&contents);
    let min_times: Vec<(&Race, Vec<i64>)> = calculate_win_hold_times(&races);

    // multiply all the lengths of the vectors
    let product = min_times.iter().fold(1, |acc, (_, v)| acc * v.len());

    println!("Product: {}", product);
}

// time:7 distance:9
fn calculate_win_hold_times(races: &Vec<Race>) -> Vec<(&Race, Vec<i64>)> {
    let mut win_hold_times_per_race: Vec<(&Race, Vec<i64>)> = Vec::new();

    for race in races {
        let mut win_hold_times: Vec<i64> = Vec::new();
        // hold time h
        // speed = (1 * h) m/s
        // distance d = (t - h) * (h)
        // 0 = h^2 - (t * h) + d
        let hold_time = find_hold_time(race.time as f64, race.best_distance as f64).unwrap();

        let min_time = (hold_time.0 + 0.01).ceil();
        let max_time = (hold_time.1 - 0.01).ceil();
        println!("min_time: {} max_time: {}", min_time, max_time);

        // get all integers between min and max
        for i in min_time as i64..max_time as i64 {
            win_hold_times.push(i);
        }
        win_hold_times_per_race.push((race, win_hold_times))
    }

    win_hold_times_per_race
}
// quadratic formula to find hold time
fn find_hold_time(total_time: f64, distance: f64) -> Option<(f64, f64)> {
    let a = 1.0;
    let b = -total_time;
    let c = distance;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None // No real solutions
    } else {
        // Calculate the two possible solutions
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);

        // return the pairs in order of smallest to largest
        if root1 > root2 {
            return Some((root2, root1));
        }

        Some((root1, root2))
    }
}

fn parse_races(contents: &Vec<String>) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();
    let times: Vec<i64> = contents[0]
        .splitn(2, "Time:")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    let distances: Vec<i64> = contents[1]
        .splitn(2, "Distance:")
        .nth(1)
        .unwrap()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    for i in 0..times.len() {
        races.push(Race { time: times[i], best_distance: distances[i] });
    }

    races
}

fn read_file(file_path: &str) -> Vec<String> {
    let file = env::current_dir().unwrap().join(file_path);
    let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
    return contents.split('\n').map(|s| s.to_string()).collect();
}