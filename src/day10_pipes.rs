use std::{cmp, collections::VecDeque, str::FromStr};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipeGrid {
    locations: Vec<Vec<PipeLocation>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PipeLocation {
    kind: PipeKind,
    distance: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeKind {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl PipeLocation {
    pub fn is_vertex(&self) -> bool {
        self.distance.is_some()
            && matches!(
                self.kind,
                PipeKind::NorthEast
                    | PipeKind::NorthWest
                    | PipeKind::SouthWest
                    | PipeKind::SouthEast
            )
    }
}

impl PipeGrid {
    pub fn find_max_distance(&mut self) -> u32 {
        let mut bfs_nodes = VecDeque::new();
        let start = self.start_location();
        bfs_nodes.push_back(start);

        self.locations[start.0][start.1].distance = Some(0);

        let mut max_distance = 0;

        while let Some(search_loc) = bfs_nodes.pop_front() {
            let mut next_nodes = self.search_locations(search_loc);

            let next_dist = self.locations[search_loc.0][search_loc.1].distance.unwrap() + 1;

            // filter for nodes that connect back to current loc
            next_nodes.retain(|&loc| {
                self.search_locations(loc).contains(&search_loc)
                    && self.locations[loc.0][loc.1].distance.is_none()
            });

            if !next_nodes.is_empty() {
                max_distance = cmp::max(max_distance, next_dist);
            }

            for (col, row) in next_nodes {
                self.locations[col][row].distance = Some(next_dist);
                bfs_nodes.push_back((col, row));
            }
        }

        max_distance
    }

    pub fn points_inside_pipe(&mut self) -> u32 {
        self.find_max_distance();
        let vertices = self.vertices();
        let area = shoelace(vertices.iter().copied());

        // pick's theorem:
        // A = i + b/2 - 1
        // we have A and number of boundary points, solve for interior points
        // i = A - b/2 + 1

        (area + 1 - (vertices.len() / 2)) as u32
    }

    pub fn print_grid(&self) {
        for row in &self.locations {
            for loc in row {
                if let Some(dist) = loc.distance {
                    print!("{dist}");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn start_location(&self) -> (usize, usize) {
        let n_rows = self.locations[0].len();
        let flat_loc = self
            .locations
            .iter()
            .flatten()
            .enumerate()
            .find_map(|(i, loc)| {
                if loc.kind == PipeKind::Start {
                    Some(i)
                } else {
                    None
                }
            })
            .expect("couldn't find start location");

        let col = flat_loc / n_rows;
        let row = flat_loc % n_rows;

        (col, row)
    }

    fn search_locations(&self, location: (usize, usize)) -> Vec<(usize, usize)> {
        let (col, row) = location;
        let offsets = match self.locations[col][row].kind {
            PipeKind::Start => vec![(-1, 0), (0, 1), (0, -1), (1, 0)],
            PipeKind::NorthSouth => vec![(-1, 0), (1, 0)],
            PipeKind::EastWest => vec![(0, -1), (0, 1)],
            PipeKind::NorthEast => vec![(-1, 0), (0, 1)],
            PipeKind::NorthWest => vec![(-1, 0), (0, -1)],
            PipeKind::SouthEast => vec![(1, 0), (0, 1)],
            PipeKind::SouthWest => vec![(1, 0), (0, -1)],
            PipeKind::Ground => vec![],
        };

        offsets
            .into_iter()
            .filter_map(|(dy, dx)| {
                let y = col as i16 + dy;
                let x = row as i16 + dx;

                if y >= 0
                    && y < self.locations.len() as i16
                    && x >= 0
                    && x < self.locations[0].len() as i16
                {
                    Some((y as usize, x as usize))
                } else {
                    None
                }
            })
            .collect()
    }

    fn vertices(&mut self) -> Vec<(usize, usize)> {
        // trace pipe loop and yield vertices
        let start = self.start_location();
        let (mut last, mut current) = (None, start);

        let mut vertices = vec![];

        loop {
            if self.locations[current.0][current.1].is_vertex() {
                vertices.push(current);
            }
            let next = self
                .search_locations(current)
                .into_iter()
                .find(|loc| Some(*loc) != last && self.search_locations(*loc).contains(&current));
            last = Some(current);
            current = next.expect("couldn't find next node");
            if current == start {
                break;
            }
        }
        println!("vertices {vertices:?}");

        vertices
    }

    fn fill_in_start_pipe(&mut self) -> (usize, usize) {
        let (row, col) = self.start_location();

        (row, col)
    }
}

fn shoelace(coords: impl Iterator<Item = (usize, usize)> + ExactSizeIterator + Clone) -> usize {
    let first_term = expand_shoelace(coords.clone(), false);
    let second_term = expand_shoelace(coords.clone(), true);

    if first_term >= second_term {
        (first_term - second_term) / 2
    } else {
        (second_term - first_term) / 2
    }
}

fn expand_shoelace(
    coords: impl Iterator<Item = (usize, usize)> + ExactSizeIterator + Clone,
    start_with_x: bool,
) -> usize {
    // (x1y2) + (x2y3) + (x3y4) + ...

    // loop around to capture y1 term
    let n_terms = coords.len() + 1;

    coords
        .cycle()
        .take(n_terms)
        .tuple_windows()
        .map(
            |((x1, y1), (x2, y2))| {
                if start_with_x {
                    x1 * y2
                } else {
                    y1 * x2
                }
            },
        )
        .sum()
}

impl From<char> for PipeKind {
    // | is a vertical pipe connecting north and south.
    // - is a horizontal pipe connecting east and west.
    // L is a 90-degree bend connecting north and east.
    // J is a 90-degree bend connecting north and west.
    // 7 is a 90-degree bend connecting south and west.
    // F is a 90-degree bend connecting south and east.
    // . is ground; there is no pipe in this tile.
    // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("invalid pipe character"),
        }
    }
}

fn parse_line(line: &str) -> Vec<PipeLocation> {
    line.chars()
        .map(|c| PipeLocation {
            kind: c.into(),
            distance: None,
        })
        .collect_vec()
}

impl FromStr for PipeGrid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            locations: s.lines().map(parse_line).collect_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shoelace() {
        let input = vec![(1, 2), (3, 7), (5, 4)];

        // 1/2 ((x1y2 + x2y3 + x3y1) - (y1x2 + y2x3 + y3x1))
        // 1/2 ((1*7  + 3*4  + 5*2 ) - ( 2*3 +  7*5 +  4*1))
        // 1/2 (( 7   +  12  +  10 ) - (  6  +  35  +   4 ))
        // 1/2 (         29        ) - (        45        ))

        let first_term = expand_shoelace(input.iter().copied(), true);
        assert_eq!(first_term, 29);
        let second_term = expand_shoelace(input.iter().copied(), false);
        assert_eq!(second_term, 45);

        let shoelace = shoelace(input.iter().copied());
        assert_eq!(shoelace, 8)
    }
}
