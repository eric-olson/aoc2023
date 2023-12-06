use std::str::FromStr;

use advent_of_code::day2_cubes::{CubeSet, Game};

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = input
        .lines()
        .map(|line| Game::from_str(line).expect("failed to parse game"))
        .collect::<Vec<_>>();

    let input = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };

    let summed_possible_game_ids: u32 = games
        .iter()
        .filter(|game| game.is_possible(input))
        .map(|game| game.number)
        .sum();

    Some(summed_possible_game_ids)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input
        .lines()
        .map(|line| Game::from_str(line).expect("failed to parse game"))
        .collect::<Vec<_>>();

    Some(games.iter().map(|game| game.power_of_min_set()).sum())
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
        assert_eq!(result, Some(2286));
    }
}
