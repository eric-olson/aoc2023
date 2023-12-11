use advent_of_code::day6_boats::{parse_day6_input, parse_day6_input_pt2};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let races = parse_day6_input(input);

    let winning_product = races
        .iter()
        .fold(1, |acc, race| acc * race.number_of_winning_combos());

    Some(winning_product)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse_day6_input_pt2(input).number_of_winning_combos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
