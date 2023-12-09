use std::str::FromStr;

use advent_of_code::day3_schematic::Schematic;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let schematic = Schematic::from_str(input).unwrap();

    Some(schematic.sum_of_part_numbers())
}

pub fn part_two(input: &str) -> Option<u32> {
    let schematic = Schematic::from_str(input).unwrap();

    Some(schematic.sum_of_gear_ratios())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
