use std::{env, fs};
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};

struct Mapping {
    mappings: Vec<(i64, i64, i64)>
}

fn main() {
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

    let file = env::current_dir().unwrap().join("day5/src/input.txt");
    let content = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines: Vec<String> = content.split("\r\n").filter(|&line| !line.is_empty()).map(String::from).collect();

    // part1(&lines, &delimiters);
    part2(&lines, &delimiters);

    println!("Done!");
}

fn part2(lines: &Vec<String>, delimiters: &Vec<&str>) {
    // create pairs of seeds

    // pair each seeds
    println!("Pairing seeds...");
    let pairs: Vec<(i64, i64)> = parse_seeds_part2(&lines[0]);
    println!("Parsing maps...");
    let maps: Vec<Mapping> = get_mappings(&lines, &delimiters);

    // use parallel iterator
    let results: Vec<Vec<i64>> = pairs.par_iter().map(|pair| {
        println!("Starting a vector of seeds...");
        let a = pair.0;
        let b = pair.1;
        // create the seeds vector
        println!("Creating a vector of seeds...");
        let seeds = (0..b).into_par_iter()
            .map(|i| a + i)
            .collect::<Vec<i64>>();
        println!("Finished a vector of seeds...");

        let locations: Vec<Vec<i64>> = get_locations_from_seed(&maps, &seeds);
        println!("Finished a vector of seeds...");
        // return the last vector
        locations[locations.len() - 1].to_vec()
    }).collect();

    // get the smallest number in the last vector
    let smallest: &i64 = results.iter().flatten().min().unwrap();
    println!("Smallest Number: {}", smallest);
}

fn parse_seeds_part2(seeds_string: &String) -> Vec<(i64, i64)> {
    let seeds: Vec<i64> = parse_seeds_part1(seeds_string);
    return seeds.chunks(2).filter_map(|chunk| {
        if chunk.len() == 2 {
            Some((chunk[0], chunk[1]))
        } else {
            None
        }
    }).collect();
}

fn part1(lines: &Vec<String>, delimiters: &Vec<&str>) {
// create seeds vector
    let seeds: Vec<i64> = parse_seeds_part1(&lines[0]);

    // get all the numbers
    let maps: Vec<Mapping> = get_mappings(&lines, &delimiters);
    let locations: Vec<Vec<i64>> = get_locations_from_seed(&maps, &seeds);

    // get the smallest number in the last vector
    let smallest: &i64 = locations[locations.len() - 1].iter().min().unwrap();
    println!("Smallest Number: {}", smallest);
}

fn get_locations_from_seed(maps: &Vec<Mapping>, seeds: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut map_of_maps: Vec<Vec<i64>> = Vec::new();
    let mut current_map: Vec<i64> = seeds.to_vec();

    println!("Getting location of seeds...");
    for i in 0..maps.len() {
        let mut new_map: Vec<i64> = Vec::new();
        let mapping = &maps[i];

        for seed in &current_map {
            for j in 0..mapping.mappings.len() {
                let mapping_tuple = mapping.mappings[j];
                if seed >= &mapping_tuple.1 && seed < &(mapping_tuple.1 + mapping_tuple.2) {
                    let offset = seed - mapping_tuple.1;
                    new_map.push(mapping_tuple.0 + offset);
                    break;
                }
                if j == mapping.mappings.len() - 1 {
                    new_map.push(*seed);
                }
            }
        }

        current_map = new_map.clone();
        map_of_maps.push(new_map);
    }

    map_of_maps
}

fn get_mappings(lines: &Vec<String>, delimiters: &Vec<&str>) -> Vec<Mapping> {
    let mut mappings: Vec<Mapping> = Vec::new();

    for i in 0..delimiters.len() {
        let delimiter = delimiters[i];
        let mut mapping: Mapping = Mapping {
            mappings: Vec::new()
        };

        let start = lines.iter().position(|line| line.contains(delimiter)).unwrap() + 1;
        let end: usize = if delimiter == "humidity-to-location" {
            lines.len()
        } else {
            lines.iter().position(|line| line.contains(delimiters[i + 1])).unwrap()
        };

        for line in lines[start..end].iter() {
            let mut mapping_tuple: (i64, i64, i64) = (0, 0, 0);
            let mut split = line.split(" ");
            mapping_tuple.0 = split.next().unwrap().parse::<i64>().unwrap();
            mapping_tuple.1 = split.next().unwrap().parse::<i64>().unwrap();
            mapping_tuple.2 = split.next().unwrap().parse::<i64>().unwrap();

            mapping.mappings.push(mapping_tuple);
        }

        // sort by second element
        mapping.mappings.sort_by(|a, b| a.1.cmp(&b.1));
        mappings.push(mapping);
    }

    mappings
}

// seeds: 79 14 55 13
fn parse_seeds_part1(seeds_string: &String) -> Vec<i64> {
    println!("Parsing Seeds...");
    let mut seeds: Vec<i64> = Vec::new();
    seeds_string.split("seeds:").nth(1).unwrap().split(" ").filter(|&seed| !seed.is_empty()).for_each(|seed| {
        seeds.push(seed.parse::<i64>().unwrap());
    });

    seeds
}
