use std::{env, fs};

struct Mapping {
    mappings: Vec<(i64, i64, i64)>
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

    // get all the numbers
    let maps: Vec<Mapping> = get_mappings(&lines, &delimiters);
    let locations: Vec<Vec<i64>> = get_locations_from_seed(&maps, &seeds, &delimiters);

    // get the smallest number in the last vector
    let smallest: &i64 = locations[locations.len() - 1].iter().min().unwrap();
    println!("Smallest Number: {}", smallest);

    println!("Done!");
}

fn get_locations_from_seed(maps: &Vec<Mapping>, seeds: &Vec<i64>, delimiters: &Vec<&str>) -> Vec<Vec<i64>> {
    let mut map_of_maps: Vec<Vec<i64>> = Vec::new();
    let mut current_map: Vec<i64> = seeds.to_vec();

    for i in 0..maps.len() {
        let mut new_map: Vec<i64> = Vec::new();
        let mapping = &maps[i];

        for seed in &current_map {
            for j in 0..mapping.mappings.len() {
                let mapping_tuple = mapping.mappings[j];
                if seed >= &mapping_tuple.1 && seed < &(mapping_tuple.1 + mapping_tuple.2) {
                    let offset = seed - mapping_tuple.1;
                    new_map.push(mapping_tuple.0 + offset);
                    println!("{} - Mapping Found: {} -> {}, using: {} {} {}", delimiters[i], seed, mapping_tuple.0 + offset, mapping_tuple.0, mapping_tuple.1, mapping_tuple.2);
                    break;
                }
                if j == mapping.mappings.len() - 1 {
                    println!("{} - Default Mapping Used: {} -> {}", delimiters[i], seed, seed);
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
fn parse_seeds(seeds_string: &String) -> Vec<i64> {
    println!("Parsing Seeds...");
    let mut seeds: Vec<i64> = Vec::new();
    seeds_string.split("seeds:").nth(1).unwrap().split(" ").filter(|&seed| !seed.is_empty()).for_each(|seed| {
        seeds.push(seed.parse::<i64>().unwrap());
    });

    seeds
}
