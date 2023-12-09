use std::{cmp, str::FromStr};

use anyhow::{anyhow, Context};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub number: u32,
    rounds: Vec<Round>,
}

impl Game {
    pub fn is_possible(&self, input: CubeSet) -> bool {
        self.rounds.iter().all(|round| round.is_possible(input))
    }

    pub fn power_of_min_set(&self) -> u32 {
        self.rounds
            .iter()
            .fold(CubeSet::default(), |acc, round| {
                acc.maximized(&round.as_set())
            })
            .power()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CubeSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl CubeSet {
    /// Returns true if this CubeSet contains enough cubes to handle another CubeSet (as a draw)
    pub fn contains(&self, other: &CubeSet) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    /// Create a new CubeSet containing the maximum color values of two CubeSets
    pub fn maximized(&self, other: &CubeSet) -> CubeSet {
        CubeSet {
            red: cmp::max(self.red, other.red),
            green: cmp::max(self.green, other.green),
            blue: cmp::max(self.blue, other.blue),
        }
    }

    /// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together
    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    /// Add a draw to this CubeSet
    fn add_draw(&mut self, draw: ColorDraw) {
        match draw.color {
            CubeColor::Red => self.red += draw.quantity,
            CubeColor::Green => self.green += draw.quantity,
            CubeColor::Blue => self.blue += draw.quantity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorDraw {
    color: CubeColor,
    quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Round {
    colors: Vec<ColorDraw>,
}

impl Round {
    pub fn is_possible(&self, input: CubeSet) -> bool {
        input.contains(&self.as_set())
    }

    /// Get a representation of this round as a CubeSet
    pub fn as_set(&self) -> CubeSet {
        let mut set = CubeSet::default();

        for &draw in &self.colors {
            set.add_draw(draw);
        }

        set
    }
}

impl FromStr for CubeColor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "blue" => Ok(Self::Blue),
            _ => Err(anyhow!("invalid color")),
        }
    }
}

impl FromStr for ColorDraw {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "1 red"
        let (qty, color) = s
            .split_once(' ')
            .context("couldn't split ColorDraw string")?;

        Ok(Self {
            quantity: qty.parse()?,
            color: color.parse()?,
        })
    }
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "3 blue, 4 red"
        let colors: Result<_, _> = s.split(", ").map(ColorDraw::from_str).collect();
        Ok(Self { colors: colors? })
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, rounds) = s
            .split_once(": ")
            .context("failed to split game from rounds")?;

        let game_num = game.replace("Game ", "").parse::<u32>()?;

        let rounds: Result<_, _> = rounds.split("; ").map(Round::from_str).collect();

        Ok(Self {
            number: game_num,
            rounds: rounds?,
        })
    }
}
