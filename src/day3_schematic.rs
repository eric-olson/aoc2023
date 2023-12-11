use std::{cmp, str::FromStr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Schematic {
    numbers: Vec<Number>,
    /// 2D grid of bools, true if a symbol exists at that position
    symbol_grid: Vec<Vec<bool>>,
    /// 2D grid of gear markers
    gear_markers: Vec<Vec<GearMarker>>,
}

impl Schematic {
    /// Returns part numbers (numbers touching symbols)
    fn part_numbers(&self) -> impl Iterator<Item = Number> + '_ {
        self.numbers
            .iter()
            .filter(|number| number.touches_symbol(&self.symbol_grid))
            .copied()
    }

    /// Returns sum of all numbers touching symbols
    pub fn sum_of_part_numbers(&self) -> u32 {
        self.part_numbers().map(|n| n.value).sum()
    }

    /// Returns the sum of all gear ratios in the schematic
    pub fn sum_of_gear_ratios(&self) -> u32 {
        self.gear_markers
            .iter()
            .flatten()
            .filter_map(GearMarker::gear_ratio)
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Number {
    value: u32,
    y: usize,
    x_start: usize,
    x_end: usize,
}

impl Number {
    pub fn touches_symbol(&self, symbols: &[Vec<bool>]) -> bool {
        // create bounding box around number
        let top = self.y.saturating_sub(1);
        let left = self.x_start.saturating_sub(1);
        let right = cmp::min(self.x_end + 1, symbols[0].len() - 1);
        let bottom = cmp::min(self.y + 1, symbols.len() - 1);

        for y in top..=bottom {
            for x in left..=right {
                if symbols[y][x] {
                    return true;
                }
            }
        }

        false
    }

    fn mark_gears(&self, gears: &mut [Vec<GearMarker>]) {
        // create bounding box around number
        let top = self.y.saturating_sub(1);
        let left = self.x_start.saturating_sub(1);
        let right = cmp::min(self.x_end + 1, gears[0].len() - 1);
        let bottom = cmp::min(self.y + 1, gears.len() - 1);

        for y in top..=bottom {
            for x in left..=right {
                gears[y][x].visit_gear(self.value);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum GearMarker {
    /// Definitely not a gear, either because it's not a '*' or because it's touching too many nums
    #[default]
    NotGear,
    /// Could be a gear since it's a '*' but numbers haven't been fully counted yet
    MaybeGear,
    /// One number has been recorded touching this gear, this is the number
    OneNumber(u32),
    /// Two numbers have been recorded touching this gear, this is their sum
    TwoNumbers(u32),
}

impl GearMarker {
    /// handle gear logic for this position, given a number touching it
    fn visit_gear(&mut self, number: u32) {
        match self {
            Self::NotGear => (),
            Self::MaybeGear => *self = Self::OneNumber(number),
            Self::OneNumber(first) => *self = Self::TwoNumbers(*first * number),
            // if there are already two numbers, invalidate on the third
            Self::TwoNumbers(_) => *self = Self::NotGear,
        }
    }

    /// get gear ratio of this gear
    fn gear_ratio(&self) -> Option<u32> {
        if let &Self::TwoNumbers(ratio) = self {
            Some(ratio)
        } else {
            None
        }
    }
}

/// Parse a line, and get position of all numbers and symbols
fn parse_line(line: &str, line_number: usize) -> (Vec<Number>, Vec<bool>, Vec<GearMarker>) {
    let mut numbers = vec![];
    let mut symbols = vec![false; line.len()];
    let mut gears = vec![GearMarker::NotGear; line.len()];

    let mut current_number: Option<u32> = None;

    for (pos, c) in line.chars().enumerate() {
        if let Some(num) = c.to_digit(10) {
            if let Some(current) = &mut current_number {
                *current *= 10;
                *current += num;
            } else {
                current_number = Some(num)
            }
        } else {
            // finalize number that's ended
            if let Some(number) = current_number.take() {
                let number_len = number.checked_ilog10().unwrap_or(0) + 1;
                numbers.push(Number {
                    value: number,
                    y: line_number,
                    x_start: pos - number_len as usize,
                    x_end: pos - 1,
                })
            }

            if c != '.' {
                symbols[pos] = true;

                if c == '*' {
                    gears[pos] = GearMarker::MaybeGear;
                }
            }
        }
    }

    // handle number that goes to end of line
    if let Some(number) = current_number.take() {
        let number_len = number.checked_ilog10().unwrap_or(0) + 1;
        numbers.push(Number {
            value: number,
            y: line_number,
            x_start: line.len() - number_len as usize,
            x_end: line.len() - 1,
        })
    }

    (numbers, symbols, gears)
}

impl FromStr for Schematic {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schematic = Schematic {
            numbers: vec![],
            symbol_grid: vec![],
            gear_markers: vec![],
        };

        for (line_number, line) in s.lines().enumerate() {
            let (numbers, symbols, gears) = parse_line(line, line_number);
            schematic.numbers.extend(numbers.iter());
            schematic.symbol_grid.push(symbols);
            schematic.gear_markers.push(gears);
        }

        // mark gear ratios in schematic
        for number in &schematic.numbers {
            number.mark_gears(&mut schematic.gear_markers);
        }

        Ok(schematic)
    }
}
