use std::collections::HashSet;
use utils::solution::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            unique_houses(input.chars().map(|ch| Direction::try_from(ch).unwrap()))
                .len()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let directions = input.chars().map(|ch| Direction::try_from(ch).unwrap());
        let santa = unique_houses(directions.clone().step_by(2));
        let robosanta = unique_houses(directions.skip(1).step_by(2));

        let union: HashSet<&(i32, i32)> = santa.union(&robosanta).collect();
        Some((union.len()).to_string())
    }
}

fn unique_houses<T: Iterator<Item = Direction>>(it: T) -> HashSet<(i32, i32)> {
    let mut location = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(location);

    for direction in it {
        location = match direction {
            Direction::Up => (location.0, location.1 + 1),
            Direction::Down => (location.0, location.1 - 1),
            Direction::Left => (location.0 - 1, location.1),
            Direction::Right => (location.0 + 1, location.1),
        };
        visited.insert(location);
    }

    visited
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '>' => Ok(Direction::Left),
            '<' => Ok(Direction::Right),
            _ => Err("Invalid character"),
        }
    }
}

utils::verify!(Day03, utils::my_input!("2015", "03"), "2565", "2639");
