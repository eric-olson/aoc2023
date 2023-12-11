use advent_of_code::day5_almanac::Input;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut parsed = input.parse::<Input>().expect("failed to parse");

    let min_loc = parsed.min_location();

   Some(min_loc)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = input.parse::<Input>().expect("failed to parse");
    
    // parsed.expand_input_pairs();
    let min_loc = parsed.min_location_range();

   Some(min_loc)
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
