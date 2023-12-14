use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines().map(map_part_one).sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_amount_map: HashMap<usize, u32> = HashMap::new();
    for idx in 0..input.lines().count() {
        card_amount_map.insert(idx, 1);
    }

    input.lines().enumerate().for_each(|(i, line)| {
        let cards_won = map_part_two(line);

        for j in 1..=cards_won {
            let card_index = &i + j as usize;
            let current_card_amount = card_amount_map.get(&i).unwrap();
            card_amount_map.insert(
                card_index,
                card_amount_map.get(&card_index).unwrap() + (1 * current_card_amount),
            );
        }
    });

    Some(card_amount_map.values().sum())
}

fn map_part_one(line: &str) -> u32 {
    let (winning_numbers, owned_numbers) = extract_numbers(line);

    let mut winning_points = 0;
    for number in owned_numbers {
        if winning_numbers.contains(&number) {
            if winning_points == 0 {
                winning_points = 1;
            } else {
                winning_points = winning_points * 2;
            }
        }
    }

    winning_points
}

fn map_part_two(line: &str) -> u32 {
    let (winning_numbers, owned_numbers) = extract_numbers(line);

    owned_numbers
        .iter()
        .map(|num| if winning_numbers.contains(&num) { 1 } else { 0 })
        .sum()
}

fn extract_numbers(line: &str) -> (Vec<&str>, Vec<&str>) {
    line.split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .map(|str| str.split_whitespace().collect())
        .collect_tuple()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
