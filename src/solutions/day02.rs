use std::str::FromStr;

use advent_utils::solution::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(pilot_to_final_destination(input, update_direction).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(pilot_to_final_destination(input, update_direction_with_aim).to_string())
    }
}

fn pilot_to_final_destination(
    input: &str,
    update_algorithm: fn(Direction, SubCommand) -> Direction,
) -> i32 {
    input
        .lines()
        .flat_map(SubCommand::from_str)
        .fold(Direction::default(), update_algorithm)
        .position()
}

fn update_direction(dir: Direction, cmd: SubCommand) -> Direction {
    match cmd {
        SubCommand::Down(n) => Direction {
            depth: dir.depth + n,
            ..dir
        },
        SubCommand::Forward(n) => Direction {
            horizontal_position: dir.horizontal_position + n,
            ..dir
        },
        SubCommand::Up(n) => Direction {
            depth: dir.depth - n,
            ..dir
        },
    }
}

fn update_direction_with_aim(dir: Direction, cmd: SubCommand) -> Direction {
    match cmd {
        SubCommand::Down(n) => Direction {
            aim: dir.aim + n,
            ..dir
        },
        SubCommand::Forward(n) => Direction {
            horizontal_position: dir.horizontal_position + n,
            depth: dir.depth + dir.aim * n,
            ..dir
        },
        SubCommand::Up(n) => Direction {
            aim: dir.aim - n,
            ..dir
        },
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Direction {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl Direction {
    fn position(&self) -> i32 {
        self.horizontal_position * self.depth
    }
}

#[derive(Clone, Copy, Debug)]
enum SubCommand {
    Down(i32),
    Forward(i32),
    Up(i32),
}

impl std::str::FromStr for SubCommand {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once(' ') {
            Ok(match a {
                "forward" => Self::Forward(b.parse()?),
                "down" => Self::Down(b.parse()?),
                "up" => Self::Up(b.parse()?),
                _ => return Err(format!("Invalid token ({})", a).into()),
            })
        } else {
            Err(String::from("Could not split string").into())
        }
    }
}
