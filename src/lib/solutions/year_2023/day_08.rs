use std::{collections::HashMap, str::FromStr};

use crate::utils::solution::Solution;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (directions, network) = parse_input(input);
        Some(network.human_path_length(directions).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let (directions, network) = parse_input(input);
        Some(network.ghost_path_length(directions).to_string())
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Unexpected character"),
        }
    }
}

#[derive(Clone, Debug)]
enum EndCriterion {
    HumanEnding(String),
    GhostEnding(char),
}

impl EndCriterion {
    fn should_end(&self, state: &str) -> bool {
        match self {
            Self::HumanEnding(s) => state == s,
            Self::GhostEnding(ch) => state.ends_with(*ch),
        }
    }
}

#[derive(Clone, Debug)]
struct Network(HashMap<String, [String; 2]>);

impl Network {
    fn get(&self, s: &str, direction: Direction) -> &str {
        &self.0[&s.to_string()][match direction {
            Direction::Left => 0,
            Direction::Right => 1,
        }]
    }

    fn human_path_length<T>(&self, directions: T) -> usize
    where
        T: IntoIterator<Item = Direction>,
        <T as IntoIterator>::IntoIter: Clone,
    {
        const START: &str = "AAA";
        const END: &str = "ZZZ";
        self.path_length(
            START,
            EndCriterion::HumanEnding(END.to_string()),
            directions,
        )
    }

    fn ghost_path_length<T>(&self, directions: T) -> usize
    where
        T: IntoIterator<Item = Direction> + Clone,
        <T as IntoIterator>::IntoIter: Clone,
    {
        let startpoints = self.0.keys().filter(|k| k.ends_with('A'));
        startpoints
            .map(|start| {
                self.path_length(start, EndCriterion::GhostEnding('Z'), directions.clone())
            })
            .fold(1, num::integer::lcm)
    }

    fn path_length<T: IntoIterator<Item = Direction>>(
        &self,
        start: &str,
        end: EndCriterion,
        directions: T,
    ) -> usize
    where
        <T as IntoIterator>::IntoIter: Clone,
    {
        directions
            .into_iter()
            .cycle()
            .scan(String::from(start), |state, direction| {
                if end.should_end(state) {
                    None
                } else {
                    *state = self.get(state, direction).to_string();
                    Some(state.clone())
                }
            })
            .count()
    }
}

impl FromStr for Network {
    type Err = crate::utils::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    let (from, to) = line.split_once('=').unwrap();
                    let (l, r) = to
                        .trim()
                        .trim_matches(|ch| ch == '(' || ch == ')')
                        .split_once(',')
                        .unwrap();

                    (
                        from.trim().to_string(),
                        [l.trim().to_string(), r.trim().to_string()],
                    )
                })
                .collect(),
        ))
    }
}

fn parse_input(s: &str) -> (Vec<Direction>, Network) {
    let (directions_str, network_str) = s.split_once("\n\n").unwrap();
    let directions = directions_str.chars().map(Direction::from);

    (
        directions.collect(),
        Network::from_str(network_str).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

        let (directions, network) = parse_input(INPUT);
        assert_eq!(network.human_path_length(directions), 2);

        const GHOST_INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

        let (ghost_directions, ghost_network) = parse_input(GHOST_INPUT);
        assert_eq!(ghost_network.ghost_path_length(ghost_directions), 6);
    }
}

crate::verify!(
    Day08,
    crate::my_input!("2023", "Day08"),
    "18023",
    "14449445933179"
);
