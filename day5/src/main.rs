use std::{env, fs};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rayon::prelude::*;

struct Mapping {
    name: String,
    mapping: HashMap<i64, i64>
}

fn main() {
    let file = env::current_dir().unwrap().join("day5/src/input.txt");
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines: Vec<String> = content.split("\r\n").filter(|&line| !line.is_empty()).map(String::from).collect();

    // create seeds vector
    let seeds: Vec<i64> = parse_seeds(&lines[0]);

    // create hashmap seed-to-soil
    let delimiters: Vec<&str> = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location"
    ];

    let maps: Vec<Mapping> = delimiters.par_iter().map(|delimiter| {
        println!("Parsing Maps: {}", delimiter);
        let start = lines.iter().position(|line| line.contains(delimiter)).unwrap() + 1;

        let end: usize = if delimiter == &"humidity-to-location" {
            lines.len()
        } else {
            lines.iter().position(|line| line.contains(delimiters[delimiters.len() - 1])).unwrap()
        };

        let map: Arc<Mutex<HashMap<i64, i64>>> = Arc::new(Mutex::new(HashMap::new()));

        lines[start..end].par_iter().for_each(|line| {
            let numbers: Vec<i64> = line.split_whitespace().filter_map(|number| number.parse().ok()).collect();


            if numbers.len() >= 3 {
                let dest = numbers[0];
                let src = numbers[1];
                let range = numbers[2];

                let mut map_inner = map.lock().unwrap();
                map_inner.insert(src, dest);
                println!("Parsing Map {}: {} -> {}, with range {}", delimiter, src, dest, range);
                for i in 0..range {
                    map_inner.insert(src + i, dest + i);
                }
            }
        });

        println!("Parsed Map: {}", delimiter);
        Mapping {
            name: String::from(*delimiter),
            mapping: Arc::try_unwrap(map).unwrap().into_inner().unwrap(),
        }
    }).collect();

    let locations: Vec<i64> = seeds.par_iter().map(|seed| get_location_from_seed(*seed, &maps)).collect();
    // print min
    println!("{:?}", locations.par_iter().min().unwrap());
}

fn get_location_from_seed(seed: i64, maps: &Vec<Mapping>) -> i64 {
    let mut key = seed;
    for map in maps {
        if map.mapping.contains_key(&key) {
            key = map.mapping.get(&key).unwrap().clone();
        }
    }
    key
}

// seeds: 79 14 55 13
fn parse_seeds(seeds_string: &String) -> Vec<i64> {
    println!("Parsing Seeds...");
    let mut seeds: Vec<i64> = Vec::new();
    seeds_string.split("seeds:").nth(1).unwrap().split(" ").filter(|&seed| !seed.is_empty()).for_each(|seed| {
        seeds.push(seed.parse::<i64>().unwrap());
    });

    seeds
}
