use advent_of_code::day8_maps::Input;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let parsed = input.parse::<Input>().unwrap();

    Some(parsed.path_steps_pt1())
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = input.parse::<Input>().unwrap();

    Some(parsed.path_steps_pt2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(6));
    }
}
