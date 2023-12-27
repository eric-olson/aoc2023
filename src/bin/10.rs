use advent_of_code::day10_pipes::PipeGrid;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: PipeGrid = input.parse().unwrap();

    let max = grid.find_max_distance();

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: PipeGrid = input.parse().unwrap();

    Some(grid.points_inside_pipe())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(8));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(4));
    }
}
