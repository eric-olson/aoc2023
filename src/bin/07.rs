advent_of_code::solution!(7);

use advent_of_code::day7_cards::Hand;

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|line| line.parse().unwrap()).collect();
    hands.sort();

    Some(
        hands
            .into_iter()
            .enumerate()
            .map(|(i, hand)| hand.bid() * (i + 1) as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input.lines().map(|line| line.parse().unwrap()).collect();
    hands.iter_mut().for_each(Hand::use_jokers);
    hands.sort();

    Some(
        hands
            .into_iter()
            .enumerate()
            .map(|(i, hand)| hand.bid() * (i + 1) as u32)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
