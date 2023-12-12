use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone)]
pub struct Node {
    pub last_char: char,
    pub left: usize,
    pub right: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Input {
    directions: Vec<Direction>,
    nodes: Vec<Node>,
    pt1_start: usize,
    pt1_end: usize,
    pt2_starts: Vec<usize>,
}

impl Input {
    pub fn path_steps_pt1(&self) -> u32 {
        let mut loc = self.pt1_start;

        for (count, direction) in self.directions.iter().cycle().enumerate() {
            if loc == self.pt1_end {
                return count as u32;
            }

            loc = match direction {
                Direction::Left => self.nodes[loc].left,
                Direction::Right => self.nodes[loc].right,
            };
        }

        0
    }

    pub fn path_steps_pt2(&self) -> u64 {
        self.pt2_starts
            .iter()
            .map(|&start| self.steps_to_z(start))
            .reduce(num::integer::lcm)
            .unwrap()
    }

    fn steps_to_z(&self, start: usize) -> u64 {
        let mut loc = start;

        for (count, direction) in self.directions.iter().cycle().enumerate() {
            if self.nodes[loc].last_char == 'Z' {
                return count as u64;
            }

            loc = match direction {
                Direction::Left => self.nodes[loc].left,
                Direction::Right => self.nodes[loc].right,
            };
        }

        0
    }
}

fn parse_locations(input: &str) -> HashMap<&str, usize> {
    let mut names = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let (name, _) = line
            .split_once(" = ")
            .expect("couldn't split name from line");
        names.insert(name, i);
    }

    names
}

fn parse_nodes(input: &str, name_locations: &HashMap<&str, usize>) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(name_locations.len());

    for line in input.lines() {
        // "AAA = (BBB, CCC)"
        let (name, connections) = line
            .split_once(" = ")
            .expect("couldn't split name from line");

        let connections = connections
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap();

        let (left, right) = connections
            .split_once(", ")
            .expect("couldn't split connections");

        let left_idx = *name_locations.get(left).expect("missing left location");
        let right_idx = *name_locations.get(right).expect("missing right location");

        nodes.push(Node {
            last_char: name.chars().last().unwrap(),
            left: left_idx,
            right: right_idx,
        });
    }

    nodes
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("invalid direction"),
        })
        .collect()
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (directions, nodes) = s.split_once("\n\n").unwrap();

        let directions = parse_directions(directions);

        let name_locations = parse_locations(nodes);
        let nodes = parse_nodes(nodes, &name_locations);

        let pt1_start = *name_locations.get("AAA").unwrap_or(&0);
        let pt1_end = *name_locations.get("ZZZ").unwrap_or(&0);

        let pt2_starts = name_locations
            .iter()
            .filter_map(|(&key, &value)| {
                if key.ends_with('A') {
                    Some(value)
                } else {
                    None
                }
            })
            .collect();

        Ok(Self {
            directions,
            nodes,
            pt1_start,
            pt1_end,
            pt2_starts,
        })
    }
}
