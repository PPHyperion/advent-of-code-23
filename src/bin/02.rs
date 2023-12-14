use regex::Regex;
#[macro_use]
extern crate lazy_static;
advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let result: u32 = input.lines().map(map_part_one).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

lazy_static! {
    static ref REGEX_RED: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref REGEX_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref REGEX_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
}

fn map_part_one(line: &str) -> u32 {
    let game_number = Regex::new(r"Game (\d+):")
        .unwrap()
        .captures(line)
        .unwrap()
        .get(1)
        .map(|num| num.as_str().parse::<u32>().unwrap())
        .unwrap();
    let cube_sets: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1]
        .split("; ")
        .collect();

    let mut valid = true;

    cube_sets.iter().for_each(|set| {
        let (red, green, blue) = extract_cubes(set);
        if red > 12 || green > 13 || blue > 14 {
            valid = false;
        }
    });

    if !valid {
        return 0;
    }

    game_number
}

fn extract_cubes(set: &&str) -> (u32, u32, u32) {
    let red = REGEX_RED.captures(set).map_or(0, |caps| {
        caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
    });
    let green = REGEX_GREEN.captures(set).map_or(0, |caps| {
        caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
    });
    let blue = REGEX_BLUE.captures(set).map_or(0, |caps| {
        caps.get(1).unwrap().as_str().parse::<u32>().unwrap()
    });
    (red, green, blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
