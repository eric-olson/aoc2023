advent_of_code::solution!(1);

const NUMBER_STRINGS: &[(&str, u32)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn extract_line_num(line: &str) -> u32 {
    let mut filter = line.chars().filter_map(|c| c.to_digit(10));

    // consume first and last values from filter (it's possible they're the same value)
    let first = filter.clone().next().unwrap();
    let last = filter.next_back().unwrap();

    first * 10 + last
}

fn extract_text_numbers(line: &str) -> u32 {
    let mut first_number_pos = line.find(|c: char| c.is_ascii_digit());
    let mut first_number =
        first_number_pos.map(|pos| line.chars().nth(pos).unwrap().to_digit(10).unwrap());

    let mut last_number_pos = line.rfind(|c: char| c.is_ascii_digit());
    let mut last_number =
        last_number_pos.map(|pos| line.chars().nth(pos).unwrap().to_digit(10).unwrap());

    for &(text, number) in NUMBER_STRINGS {
        if let Some(pos) = line.find(text) {
            if pos <= first_number_pos.unwrap_or(pos) {
                first_number = Some(number);
                first_number_pos = Some(pos)
            }
        }

        if let Some(pos) = line.rfind(text) {
            if pos >= last_number_pos.unwrap_or(pos) {
                last_number = Some(number);
                last_number_pos = Some(pos)
            }
        }
    }

    first_number.unwrap() * 10 + last_number.unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(extract_line_num).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(extract_text_numbers).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_line() {
        assert_eq!(extract_line_num("asd1f2"), 12);
        assert_eq!(extract_line_num("asd3asfb1f2"), 32);
        assert_eq!(extract_line_num("treb7uchet"), 77);
    }

    #[test]
    fn test_extract_numbers() {
        assert_eq!(extract_text_numbers("234eightsevensix8"), 28);
        assert_eq!(extract_text_numbers("1oneasdfasdf"), 11);
        assert_eq!(extract_text_numbers("oneight"), 18);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
