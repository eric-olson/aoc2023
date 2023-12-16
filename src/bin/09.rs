advent_of_code::solution!(9);

use advent_of_code::day9_oasis::{extrapolate_sequence, parse_input_line};

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(parse_input_line)
            .map(|seq| extrapolate_sequence(&seq, false))
            .sum::<i64>(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(parse_input_line)
            .map(|seq| extrapolate_sequence(&seq, true))
            .sum::<i64>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
