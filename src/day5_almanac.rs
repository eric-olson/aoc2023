use std::{iter, mem, str::FromStr};

use itertools::Itertools;

/// start, end, modified
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocationRange(u64, u64, bool);

#[derive(Debug, Clone, Copy)]
pub struct Mapping {
    dest_start: u64,
    src_start: u64,
    length: u64,
}

#[derive(Debug, Clone)]
pub struct Input {
    seed_locations: Vec<u64>,
    maps: Vec<Vec<Mapping>>,
}

impl Mapping {
    pub fn src_start(&self) -> u64 {
        self.src_start
    }

    pub fn src_end(&self) -> u64 {
        self.src_start + self.length - 1
    }

    pub fn distance(&self) -> i64 {
        self.dest_start as i64 - self.src_start as i64
    }

    pub fn transform_location(&self, location: &mut u64) -> bool {
        // if location is outside of mapping, nothing to do
        if *location < self.src_start || *location >= self.src_start + self.length {
            return false;
        }

        // modify location by difference between source and dest
        *location = (*location as i64 + self.distance()) as u64;

        true
    }
}

impl LocationRange {
    pub fn new(start: u64, len: u64) -> Self {
        Self(start, start + len - 1, false)
    }

    pub fn start(&self) -> u64 {
        self.0
    }

    pub fn end(&self) -> u64 {
        self.1
    }

    pub fn modified(&self) -> bool {
        self.2
    }

    pub fn reset_modified_flag(&mut self) {
        self.2 = false;
    }

    // apply a mapping to all locations in this range, resulting in one or more resulting ranges
    pub fn apply_mapping(&self, mapping: &Mapping) -> impl Iterator<Item = LocationRange> {
        // mapping can result in 1, 2, or 3 output ranges (not overlapping, partially overlapping, or contained within)
        let mut out_range = *self;

        let prefix = if mapping.src_start() > self.start() && mapping.src_start() <= self.end() {
            out_range.0 = mapping.src_start();
            Some(LocationRange(self.start(), mapping.src_start() - 1, false))
        } else {
            None
        };

        let suffix = if mapping.src_end() >= self.start() && mapping.src_end() < self.end() {
            out_range.1 = mapping.src_end();
            Some(LocationRange(mapping.src_end() + 1, self.end(), false))
        } else {
            None
        };

        // apply mapping to original
        if out_range.0 >= mapping.src_start() && out_range.1 <= mapping.src_end() {
            out_range.0 = (out_range.0 as i64 + mapping.distance()) as u64;
            out_range.1 = (out_range.1 as i64 + mapping.distance()) as u64;
            out_range.2 = true;
        }

        prefix
            .into_iter()
            .chain(iter::once(out_range))
            .chain(suffix)
    }

    /// apply a batch of mappings
    pub fn apply_mapping_batch(&self, batch: &[Mapping]) -> impl Iterator<Item = LocationRange> {
        let mut mapped = vec![*self];
        for mapping in batch {
            let input_range = mem::take(&mut mapped);
            for range in input_range {
                if range.modified() {
                    // preserve ranges that are already mapped
                    mapped.push(range);
                } else {
                    mapped.extend(range.apply_mapping(mapping));
                }
            }
        }

        for range in &mut mapped {
            range.reset_modified_flag();
        }

        mapped.into_iter()
    }
}

impl Input {
    // part 1 solution; modifies locations in-place
    pub fn min_location(&mut self) -> u64 {
        for location in &mut self.seed_locations {
            for map in &self.maps {
                for mapping in map {
                    if mapping.transform_location(location) {
                        break;
                    }
                }
            }
        }

        self.seed_locations.iter().copied().min().unwrap()
    }

    // brute force for part 2 solution (very slow)
    pub fn expand_input_pairs(&mut self) {
        let expanded = self
            .seed_locations
            .iter()
            .tuples()
            .flat_map(|(&start, &len)| start..start + len)
            .collect_vec();

        self.seed_locations = expanded;
    }

    // better part 2 solution that works with ranges
    pub fn min_location_range(&self) -> u64 {
        let mut ranges = self.input_ranges().collect_vec();

        for mapping_batch in &self.maps {
            let input_ranges = mem::take(&mut ranges);

            for range in input_ranges {
                ranges.extend(range.apply_mapping_batch(&mapping_batch));
            }
        }

        ranges.iter().min().expect("no minimum").0
    }

    // convert inputs into ranges
    fn input_ranges(&self) -> impl Iterator<Item = LocationRange> + '_ {
        self.seed_locations
            .iter()
            .tuples()
            .map(|(&start, &len)| LocationRange::new(start, len))
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 49 53 8
        // dest, src, length
        let [dest, src, length] = s
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .expect("failed to parse mapping");

        Ok(Self {
            dest_start: dest.parse().unwrap(),
            src_start: src.parse().unwrap(),
            length: length.parse().unwrap(),
        })
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // first line: seed locations
        let (seed_line, remainder) = s.split_once('\n').unwrap();

        let seeds = seed_line
            .split_once(":")
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(|loc| loc.parse().unwrap())
            .collect_vec();

        let mut mappings_vec = vec![];
        let mut map_vec = vec![];

        for line in remainder.lines() {
            // "seeds: 1 2 3 4"
            if line.ends_with(':') {
                if !map_vec.is_empty() {
                    mappings_vec.push(mem::take(&mut map_vec));
                }
                continue;
            }

            if line.is_empty() {
                continue;
            }

            map_vec.push(line.parse::<Mapping>().unwrap());
        }

        if !map_vec.is_empty() {
            mappings_vec.push(mem::take(&mut map_vec));
        }

        Ok(Self {
            seed_locations: seeds,
            maps: mappings_vec,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_split() {
        // range start 4, len 10 [4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
        let range = LocationRange::new(4, 10);

        // [1, 2, 3]
        let no_overlap = Mapping {
            src_start: 1,
            dest_start: 5,
            length: 3,
        };

        assert_eq!(
            range.apply_mapping(&no_overlap).collect_vec(),
            [LocationRange(4, 13, false)]
        );

        // [2, 3, 4] -> [10, 11, 12]
        let overlaps_start = Mapping {
            src_start: 2,
            dest_start: 10,
            length: 3,
        };

        assert_eq!(
            range.apply_mapping(&overlaps_start).collect_vec(),
            [LocationRange(12, 12, true), LocationRange(5, 13, false)]
        );

        // [13, 14, 15] -> [10, 11, 12]
        let overlaps_end = Mapping {
            src_start: 13,
            dest_start: 10,
            length: 3,
        };
        assert_eq!(
            range.apply_mapping(&overlaps_end).collect_vec(),
            [LocationRange(4, 12, false), LocationRange(10, 10, true)]
        );

        // [6, 7, 8] -> [10, 11, 12]
        let full_overlap = Mapping {
            src_start: 6,
            dest_start: 10,
            length: 3,
        };
        assert_eq!(
            range.apply_mapping(&full_overlap).collect_vec(),
            [
                LocationRange(4, 5, false),
                LocationRange(10, 12, true),
                LocationRange(9, 13, false)
            ]
        );
    }
}
