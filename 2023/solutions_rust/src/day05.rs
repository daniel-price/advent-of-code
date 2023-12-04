use std::collections::HashMap;

#[derive(Debug)]
struct Range {
    source_start: i128,
    destination_start: i128,
    range_length: i128,
}

fn part1(input: String) -> i128 {
    let mut seeds: Vec<i128> = Vec::new();
    let mut current_map_key = "";
    let mut maps: HashMap<&str, Vec<Range>> = HashMap::new();
    maps.insert("seed-to-soil", Vec::new());
    maps.insert("soil-to-fertilizer", Vec::new());
    maps.insert("fertilizer-to-water", Vec::new());
    maps.insert("water-to-light", Vec::new());
    maps.insert("light-to-temperature", Vec::new());
    maps.insert("temperature-to-humidity", Vec::new());
    maps.insert("humidity-to-location", Vec::new());

    input.split('\n').for_each(|line| {
        if line.starts_with("seeds: ") {
            let seeds_string = line.split("seeds: ").collect::<Vec<&str>>()[1];
            seeds_string.split(' ').for_each(|num| {
                seeds.push(num.parse::<i128>().unwrap());
            });
        } else if line.ends_with(" map:") {
            let key = line.split(" map:").collect::<Vec<&str>>()[0];
            current_map_key = key;
        } else {
            if current_map_key.is_empty() {
                return;
            }

            let current_map: &mut Vec<Range> = maps.get_mut(current_map_key).unwrap();

            let numbers = line
                .split(' ')
                .filter(|e| !e.is_empty())
                .map(|e| e.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();

            if numbers.len() != 3 {
                return;
            }

            let destination_start = numbers[0];
            let source_start = numbers[1];
            let range_length = numbers[2];

            let range = Range {
                source_start,
                destination_start,
                range_length,
            };
            current_map.push(range);
        }
    });

    seeds
        .iter()
        .map(|seed_number| {
            let soil = get_mapped_number(&maps, "seed-to-soil", seed_number);

            let fertilizer = get_mapped_number(&maps, "soil-to-fertilizer", &soil);

            let water = get_mapped_number(&maps, "fertilizer-to-water", &fertilizer);

            let light = get_mapped_number(&maps, "water-to-light", &water);

            let temperature = get_mapped_number(&maps, "light-to-temperature", &light);

            let humidity = get_mapped_number(&maps, "temperature-to-humidity", &temperature);

            get_mapped_number(&maps, "humidity-to-location", &humidity)
        })
        .min()
        .unwrap()
}

fn get_mapped_number(maps: &HashMap<&str, Vec<Range>>, map_key: &str, number: &i128) -> i128 {
    let range = maps.get(map_key).unwrap().iter().find(|range| {
        number >= &range.source_start && number < &(range.source_start + range.range_length)
    });

    match range {
        Some(range) => number + range.destination_start - range.source_start,
        None => *number,
    }
}

fn part2(input: String) -> i128 {
    let mut seeds: Vec<Range> = Vec::new();
    let mut current_map_key = "";
    let mut maps: HashMap<&str, Vec<Range>> = HashMap::new();
    maps.insert("seed-to-soil", Vec::new());
    maps.insert("soil-to-fertilizer", Vec::new());
    maps.insert("fertilizer-to-water", Vec::new());
    maps.insert("water-to-light", Vec::new());
    maps.insert("light-to-temperature", Vec::new());
    maps.insert("temperature-to-humidity", Vec::new());
    maps.insert("humidity-to-location", Vec::new());

    input.split('\n').for_each(|line| {
        if line.starts_with("seeds: ") {
            let seeds_string = line.split("seeds: ").collect::<Vec<&str>>()[1];
            let split_seeds_string = seeds_string.split(' ').collect::<Vec<&str>>();

            let mut i = 0;
            while i < split_seeds_string.len() {
                let num_start = split_seeds_string[i].parse::<i128>().unwrap();
                let range_length = split_seeds_string[i + 1].parse::<i128>().unwrap();

                seeds.push(Range {
                    source_start: num_start,
                    destination_start: num_start,
                    range_length,
                });

                i += 2;
            }
        } else if line.ends_with(" map:") {
            let key = line.split(" map:").collect::<Vec<&str>>()[0];
            current_map_key = key;
        } else {
            if current_map_key.is_empty() {
                return;
            }

            let current_map: &mut Vec<Range> = maps.get_mut(current_map_key).unwrap();

            let numbers = line
                .split(' ')
                .filter(|e| !e.is_empty())
                .map(|e| e.parse::<i128>().unwrap())
                .collect::<Vec<i128>>();

            if numbers.len() != 3 {
                return;
            }

            let destination_start = numbers[0];
            let source_start = numbers[1];
            let range_length = numbers[2];

            let range = Range {
                source_start,
                destination_start,
                range_length,
            };
            current_map.push(range);
        }
    });

    println!("seeds: {:#?}", seeds);

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::input;

    #[test]
    fn part1sample() {
        let answer = part1(input("example", file!()));
        assert_eq!(answer, 35);
    }

    #[test]
    fn part1input() {
        let answer = part1(input("input", file!()));
        assert_eq!(answer, 165788812);
    }

    #[test]
    fn part2sample() {
        let answer = part2(input("example", file!()));
        assert_eq!(answer, 46);
    }

    #[test]
    fn part2input() {
        let answer = part2(input("input", file!()));
        assert_eq!(answer, 0);
    }
}
