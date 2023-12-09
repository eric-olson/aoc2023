use std::str::FromStr;

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Card {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
    copies: u32,
}

impl Card {
    pub fn point_value(&self) -> u32 {
        match self.matches() {
            0 => 0,
            n => 2u32.pow(n - 1)
        }
    }

    pub fn matches(&self) -> u32 {
        let mut matches = 0;

        for winning_number in &self.winning_numbers {
            for number in &self.numbers_you_have {
                if number == winning_number {
                    if matches == 0 {
                        matches = 1
                    } else {
                        matches += 1;
                    }
                }
            }
        }

        matches
    }

    pub fn copies(&self) -> u32 {
        self.copies
    }

    pub fn add_copies(&mut self, n_copies: u32) {
        self.copies += n_copies;
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut card = Self {
            winning_numbers: vec![],
            numbers_you_have: vec![],
            copies: 1,
        };
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let (_card, all_numbers) = s.split_once(':').context("splitting card from numbers")?;

        let (numbers, winners) = all_numbers
            .split_once("|")
            .context("splitting numbers from winners")?;

        for number in numbers.split_ascii_whitespace() {
            card.winning_numbers
                .push(number.parse::<u32>().context("failed to parse number")?);
        }

        for winner in winners.split_ascii_whitespace() {
            card.numbers_you_have
                .push(winner.parse::<u32>().context("failed to parse number")?);
        }

        Ok(card)
    }
}
