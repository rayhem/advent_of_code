use crate::solutions::{self, Solution};
use std::str::FromStr;

pub struct Day12 {}

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(set_sail(input, MovementMode::Ship(Heading::East)).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(set_sail(input, MovementMode::Waypoint((10, 1))).to_string())
    }
}

fn set_sail(input: &str, movement_mode: MovementMode) -> i32 {
    let maneuvers: Vec<_> = input
        .lines()
        .map(|line| Maneuver::from_str(line).unwrap())
        .collect();
    let mut ship = Ship::new(movement_mode);
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

impl Into<Point> for Heading {
    fn into(self) -> Point {
        match self {
            Heading::East => (1, 0),
            Heading::North => (0, 1),
            Heading::West => (-1, 0),
            Heading::South => (0, -1),
        }
    }
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

type Point = (i32, i32);
#[derive(Clone, Copy, Debug)]
enum MovementMode {
    Ship(Heading),
    Waypoint(Point),
}

#[derive(Clone, Copy, Debug)]
struct Ship {
    location: Point,
    movement_mode: MovementMode,
}

impl Ship {
    fn new(movement_mode: MovementMode) -> Self {
        Ship {
            location: (0, 0),
            movement_mode,
        }
    }

    fn maneuver(&mut self, maneuver: &Maneuver) {
        use Maneuver::*;
        match *maneuver {
            Move(heading, d) => move_by(
                match &mut self.movement_mode {
                    MovementMode::Ship(_) => &mut self.location,
                    MovementMode::Waypoint(waypoint) => waypoint,
                },
                heading.into(),
                d,
            ),
            Rotate(r) => match &mut self.movement_mode {
                MovementMode::Ship(heading) => *heading = ((*heading as i32 + r + 4) % 4).into(),
                MovementMode::Waypoint(waypoint) => {
                    let tmp = waypoint.clone();
                    match (r + 4) % 4 {
                        0 => {}
                        1 => {
                            waypoint.0 = -tmp.1;
                            waypoint.1 = tmp.0;
                        }
                        2 => {
                            waypoint.0 = -tmp.0;
                            waypoint.1 = -tmp.1;
                        }
                        3 => {
                            waypoint.0 = tmp.1;
                            waypoint.1 = -tmp.0;
                        }
                        _ => {}
                    }
                }
            },
            Forward(d) => match self.movement_mode {
                MovementMode::Ship(heading) => move_by(&mut self.location, heading.into(), d),
                MovementMode::Waypoint(waypoint) => move_by(&mut self.location, waypoint, d),
            },
        }
    }

    fn sail(&mut self, maneuvers: &[Maneuver]) {
        for maneuver in maneuvers {
            self.maneuver(maneuver);
        }
    }
}

fn move_by((ref mut x, ref mut y): &mut (i32, i32), (dx, dy): Point, distance: i32) {
    *x += dx * distance;
    *y += dy * distance;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "F10\nN3\nF7\nR90\nF11";
    const INPUT: &str =
        include_str!("/home/connor/Documents/code/advent/advent2020/inputs/day12.txt");

    #[test]
    fn example_part_one() {
        assert_eq!(Day12 {}.part_one(EXAMPLE), Some(String::from("25")));
    }

    #[test]
    fn example_part_two() {
        assert_eq!(Day12 {}.part_two(EXAMPLE), Some(String::from("286")));
    }

    #[test]
    fn baseline_part_one() {
        assert_eq!(Day12 {}.part_one(INPUT), Some(String::from("381")));
    }

    #[test]
    fn baseline_part_two() {
        assert_eq!(Day12 {}.part_two(INPUT), Some(String::from("28591")));
    }
}
