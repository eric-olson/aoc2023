use advent_of_code::day4_cards::Card;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let points = input
        .lines()
        .map(|line| {
            line.parse::<Card>()
                .expect("failed to parse card")
                .point_value()
        })
        .sum();

    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<Card> = input
        .lines()
        .map(|line| line.parse().expect("failed to parse card"))
        .collect();

    for pos in 0..cards.len() {
        let matches = cards[pos].matches() as usize;
        let copies = cards[pos].copies();

        for won_card in &mut cards.iter_mut().skip(pos + 1).take(matches) {
            won_card.add_copies(copies);
        }
    }

    Some(cards.iter().map(Card::copies).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
