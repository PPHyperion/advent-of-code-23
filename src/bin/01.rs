advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().map(extract_numbers).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: Vec<u32> = input.lines().map(map_part_two).collect();
    Some(result.iter().sum())
}

const NUMBERS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn string_to_number(str: &str) -> Option<&str> {
    match str {
        "one" => Some("1"),
        "two" => Some("2"),
        "three" => Some("3"),
        "four" => Some("4"),
        "five" => Some("5"),
        "six" => Some("6"),
        "seven" => Some("7"),
        "eight" => Some("8"),
        "nine" => Some("9"),
        _ => None,
    }
}
fn map_part_two(line: &str) -> u32 {
    let mut new_line: String = line.to_string();

    let mut ctr = 1;
    'outer: while ctr < new_line.chars().count() + 1 {
        if new_line.chars().nth(ctr - 1).unwrap().is_digit(10) {
            break;
        }
        for number in NUMBERS {
            if new_line[0..ctr].contains(number) {
                new_line = new_line.replacen(number, string_to_number(&number).unwrap(), 1);
                break 'outer;
            }
        }
        ctr += 1;
    }

    let size = new_line.chars().collect::<Vec<char>>().len();
    ctr = size - 1;
    'outer: while ctr > 0 {
        if new_line
            .chars()
            .collect::<Vec<char>>()
            .get(ctr)
            .unwrap()
            .is_digit(10)
        {
            break;
        }
        let sub = &new_line[ctr..size];
        for number in NUMBERS {
            if sub.contains(number) {
                let slice = &sub.replacen(number, string_to_number(&number).unwrap(), 1);
                new_line.replace_range(ctr..size, slice);
                break 'outer;
            }
        }
        ctr -= 1;
    }

    extract_numbers(&new_line.as_str())
}

fn extract_numbers(line: &str) -> u32 {
    let mut digit1: (usize, char) = (std::usize::MAX, '0');
    let mut digit2: (usize, char) = (std::usize::MIN, '0');

    for (i, c) in line.char_indices() {
        if !c.is_digit(10) {
            continue;
        };

        if i <= digit1.0 {
            digit1 = (i, c)
        }
        if i >= digit2.0 {
            digit2 = (i, c)
        }
    }

    [digit1.1, digit2.1]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
