advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut digi1: (usize, char) = (std::usize::MAX, '0');
        let mut digi2: (usize, char) = (std::usize::MIN, '0');

        for (i, c) in line.char_indices() {
            if !c.is_digit(10) {
                continue;
            };

            if i <= digi1.0 {
                digi1 = (i, c)
            }
            if i >= digi2.0 {
                digi2 = (i, c)
            }
        }

        let number: u32 = [digi1.1, digi2.1]
            .iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        sum += number;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
