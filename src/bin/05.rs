use std::collections::BTreeMap;

use itertools::Itertools;
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let seeds: Vec<u64> = extract_seeds(input);
    let mappings_map = create_mappings_map(input);

    seeds.iter().map(|seed| map_seed(seed, &mappings_map)).min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let seed_ranges = extract_seeds(input);
    let mappings_map = create_mappings_map(input);

    let mut ctr = 0;
    let mut location_numbers: Vec<u64> = Vec::new();
    while ctr < seed_ranges.len() - 1 {
        let mut seeds: Vec<u64> = Vec::new();
        let start = seed_ranges.get(ctr).unwrap();
        let range = seed_ranges.get(ctr + 1).unwrap();

        for num in 0..*range {
            seeds.push(start + num);
        }

        let min_result: u64 = seeds
            .iter()
            .map(|seed| map_seed(seed, &mappings_map))
            .min()
            .unwrap();
        location_numbers.push(min_result);

        ctr += 2;
    }

    location_numbers.iter().min().copied()
}

fn extract_seeds(input: &str) -> Vec<u64> {
    input
        .lines()
        .nth(0)
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect()
}

fn create_mappings_map(input: &str) -> BTreeMap<u64, Vec<(u64, u64, u64)>> {
    let mut retval: BTreeMap<u64, Vec<(u64, u64, u64)>> = BTreeMap::new();
    let mut map_idx: u64 = 0;
    let mut collect_mappings = false;
    for line in input.lines() {
        if line.contains("map") {
            map_idx += 1;
            collect_mappings = true;
            continue;
        }

        if collect_mappings {
            if line.is_empty() {
                collect_mappings = false;
                continue;
            }
            if let Some(mappings_vec) = retval.get_mut(&map_idx) {
                mappings_vec.push(extract_mappings(line))
            } else {
                retval.insert(map_idx, vec![extract_mappings(line)]);
            }
        }
    }
    retval
}

fn extract_mappings(line: &str) -> (u64, u64, u64) {
    line.split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect_tuple::<(u64, u64, u64)>()
        .unwrap()
}

fn map_seed(seed: &u64, mappings_map: &BTreeMap<u64, Vec<(u64, u64, u64)>>) -> u64 {
    let mut retval: u64 = *seed;
    'outer: for mappings in mappings_map.values().into_iter() {
        for mapping in mappings.iter() {
            if retval >= mapping.1 && retval <= (mapping.1 + mapping.2) - 1 {
                retval = if retval > mapping.1 {
                    mapping.0 + (retval - mapping.1)
                } else {
                    mapping.0 + (mapping.1 - retval)
                };
                continue 'outer;
            }
        }
    }
    retval
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
