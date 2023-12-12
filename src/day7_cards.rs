use std::{collections::BTreeMap, str::FromStr};

use anyhow::bail;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    pub fn new(cards: &[Card], bid: u32) -> Self {
        assert_eq!(cards.len(), 5, "hands must have 5 cards");
        Self {
            cards: cards.into(),
            bid,
        }
    }

    pub fn bid(&self) -> u32 {
        self.bid
    }

    pub fn use_jokers(&mut self) {
        for card in &mut self.cards {
            if *card == Card::Jack {
                *card = Card::Joker;
            }
        }
    }

    pub fn hand_type(&self) -> HandType {
        let mut card_quantities = BTreeMap::new();

        for card in &self.cards {
            *card_quantities.entry(card).or_insert(0) += 1;
        }

        let n_jokers = card_quantities.remove(&Card::Joker).unwrap_or(0);

        if n_jokers == 5 {
            return HandType::FiveOfAKind;
        }

        let mut sorted_quantities = card_quantities.into_values().sorted().collect_vec();

        // apply joker to max element
        *sorted_quantities.last_mut().unwrap() += n_jokers;

        match sorted_quantities.last().expect("missing card") {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if sorted_quantities.len() == 2 {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if sorted_quantities.len() == 3 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => panic!("invalid hand type"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.cards.cmp(&other.cards)
    }
}

impl TryFrom<char> for Card {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => bail!("inavlid card"),
        })
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').expect("failed to split hand");

        let cards = cards
            .chars()
            .map(|c| Card::try_from(c).expect("invalid card"))
            .collect();

        Ok({
            Self {
                cards,
                bid: bid.parse().expect("invalid bid"),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hand_identification() {
        assert_eq!(
            Hand::new(&[Card::Two, Card::Two, Card::Two, Card::Two, Card::Two], 0).hand_type(),
            HandType::FiveOfAKind
        );

        assert_eq!(
            Hand::new(&[Card::Ace, Card::Two, Card::Two, Card::Two, Card::Two], 0).hand_type(),
            HandType::FourOfAKind
        );

        assert_eq!(
            Hand::new(&[Card::Ace, Card::Ace, Card::Two, Card::Two, Card::Two], 0).hand_type(),
            HandType::FullHouse
        );

        assert_eq!(
            Hand::new(
                &[Card::Ace, Card::Three, Card::Two, Card::Two, Card::Two],
                0
            )
            .hand_type(),
            HandType::ThreeOfAKind
        );

        assert_eq!(
            Hand::new(
                &[Card::Ace, Card::Ace, Card::Three, Card::Three, Card::Two],
                0
            )
            .hand_type(),
            HandType::TwoPair
        );

        assert_eq!(
            Hand::new(
                &[Card::Ace, Card::Ace, Card::Three, Card::Four, Card::Two],
                0
            )
            .hand_type(),
            HandType::OnePair
        );

        assert_eq!(
            Hand::new(
                &[Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five],
                0
            )
            .hand_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn hand_ordering() {
        assert!(
            Hand::new(&[Card::Two, Card::Two, Card::Two, Card::Two, Card::Two], 0)
                > Hand::new(
                    &[Card::Two, Card::Two, Card::Two, Card::Two, Card::Three],
                    0
                )
        );

        assert!(
            Hand::new(
                &[Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                0
            ) > Hand::new(
                &[Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                0
            )
        );

        assert!(
            Hand::new(
                &[Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                0
            ) > Hand::new(
                &[Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                0
            )
        );
    }
}
