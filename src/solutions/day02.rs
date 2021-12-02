use advent_utils::solution::Solution;
use std::str::FromStr;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(pilot_to_final_destination(input, update_sub_state).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(pilot_to_final_destination(input, update_sub_state_with_aim).to_string())
    }
}

fn pilot_to_final_destination(
    input: &str,
    update_algorithm: fn(SubState, SubCommand) -> SubState,
) -> i32 {
    input
        .lines()
        .flat_map(SubCommand::from_str)
        .fold(SubState::default(), update_algorithm)
        .position()
}

fn update_sub_state(dir: SubState, cmd: SubCommand) -> SubState {
    match cmd {
        SubCommand::Down(n) => SubState {
            depth: dir.depth + n,
            ..dir
        },
        SubCommand::Forward(n) => SubState {
            horizontal_position: dir.horizontal_position + n,
            ..dir
        },
        SubCommand::Up(n) => SubState {
            depth: dir.depth - n,
            ..dir
        },
    }
}

fn update_sub_state_with_aim(dir: SubState, cmd: SubCommand) -> SubState {
    match cmd {
        SubCommand::Down(n) => SubState {
            aim: dir.aim + n,
            ..dir
        },
        SubCommand::Forward(n) => SubState {
            horizontal_position: dir.horizontal_position + n,
            depth: dir.depth + dir.aim * n,
            ..dir
        },
        SubCommand::Up(n) => SubState {
            aim: dir.aim - n,
            ..dir
        },
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct SubState {
    horizontal_position: i32,
    depth: i32,
    aim: i32,
}

impl SubState {
    fn position(&self) -> i32 {
        self.horizontal_position * self.depth
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod test {
    use super::*;
    mod unit {
        use super::*;
        static DATA: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

        #[test]
        fn parse_sub_commands() {
            use SubCommand::*;
            assert_eq!(
                DATA.lines()
                    .flat_map(SubCommand::from_str)
                    .collect::<Vec<_>>(),
                vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]
            )
        }

        #[test]
        fn pilot_algorithm() {
            assert_eq!(pilot_to_final_destination(DATA, update_sub_state), 150);
        }

        #[test]
        fn pilot_algorithm_with_aim() {
            assert_eq!(
                pilot_to_final_destination(DATA, update_sub_state_with_aim),
                900
            );
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day02 = Day02 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day02.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("1840243")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("1727785422")));
        }
    }
}
