use crate::solutions::{self, Solution};
use std::str::FromStr;

pub struct Day12 {}

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(set_sail(input).to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn set_sail(input: &str) -> i32 {
    let maneuvers: Vec<_> = input
        .lines()
        .map(|line| Maneuver::from_str(line).unwrap())
        .collect();
    let mut ship = Ship::new();
    ship.sail(&maneuvers);

    ship.location.0.abs() + ship.location.1.abs()
}

#[derive(Clone, Copy, Debug)]
enum Heading {
    East,
    North,
    West,
    South,
}

impl From<i32> for Heading {
    fn from(i: i32) -> Self {
        match i {
            0 => Heading::East,
            1 => Heading::North,
            2 => Heading::West,
            3 => Heading::South,
            _ => Heading::East,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Maneuver {
    Move(Heading, i32),
    Rotate(i32),
    Forward(i32),
}

impl FromStr for Maneuver {
    type Err = solutions::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let direction = chars.next().ok_or(solutions::Error::BadInput)?;
        let value = chars.as_str().parse()?;
        match direction {
            'N' => Ok(Self::Move(Heading::North, value)),
            'E' => Ok(Self::Move(Heading::East, value)),
            'S' => Ok(Self::Move(Heading::South, value)),
            'W' => Ok(Self::Move(Heading::West, value)),
            'L' => Ok(Self::Rotate(value / 90)),
            'R' => Ok(Self::Rotate(-value / 90)),
            'F' => Ok(Self::Forward(value)),
            _ => Err(solutions::Error::BadInput),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Ship {
    location: (i32, i32),
    heading: Heading,
}

impl Ship {
    fn new() -> Self {
        Ship {
            location: (0, 0),
            heading: Heading::East,
        }
    }

    fn maneuver(&mut self, maneuver: &Maneuver) {
        use Maneuver::*;
        match *maneuver {
            Move(heading, d) => move_by(&mut self.location, heading, d),
            Rotate(r) => self.heading = ((self.heading as i32 + r + 4) % 4).into(),
            Forward(d) => move_by(&mut self.location, self.heading, d),
        }
    }

    fn sail(&mut self, maneuvers: &[Maneuver]) {
        for maneuver in maneuvers {
            self.maneuver(maneuver);
        }
    }
}

fn move_by((ref mut x, ref mut y): &mut (i32, i32), heading: Heading, distance: i32) {
    match heading {
        Heading::East => *x += distance,
        Heading::West => *x -= distance,
        Heading::North => *y += distance,
        Heading::South => *y -= distance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        include_str!("/home/connor/Documents/code/advent/advent2020/inputs/day12.txt");

    #[test]
    fn validate_part1() {
        let day12 = Day12 {};
        assert_eq!(day12.part_one(INPUT), Some(String::from("381")));
    }
}
