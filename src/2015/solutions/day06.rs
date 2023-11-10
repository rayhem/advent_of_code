use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::str::FromStr;

use utils::solution::Solution;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> Option<String> {
        let rule = |action: &Action, x: i32| match action {
            Action::Toggle => 1 - x,
            Action::TurnOff => max(0, x - 1),
            Action::TurnOn => min(1, x + 1),
        };

        Some(total_lights(input, rule).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let rule = |action: &Action, x: i32| match action {
            Action::Toggle => x + 2,
            Action::TurnOff => max(0, x - 1),
            Action::TurnOn => x + 1,
        };

        Some(total_lights(input, rule).to_string())
    }
}

fn total_lights(input: &str, update_rule: fn(&Action, i32) -> i32) -> i32 {
    let mut grid: Grid = [[0; 1000]; 1000];
    let instructions = input.lines().map(parse_instruction);

    for (action, (row1, col1), (row2, col2)) in instructions {
        for row in row1..=row2 {
            for col in col1..=col2 {
                grid[row as usize][col as usize] =
                    update_rule(&action, grid[row as usize][col as usize]);
            }
        }
    }

    grid.iter().map(|x| x.iter().sum::<i32>()).sum::<i32>()
}

fn parse_instruction(s: &str) -> (Action, (i32, i32), (i32, i32)) {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d+),(\\d+) through (\\d+),(\\d+)").unwrap();
    }

    let captures = RE.captures(s).unwrap();
    (
        Action::from_str(s).unwrap(),
        (
            captures[1].parse::<i32>().unwrap(),
            captures[2].parse::<i32>().unwrap(),
        ),
        (
            captures[3].parse::<i32>().unwrap(),
            captures[4].parse::<i32>().unwrap(),
        ),
    )
}

type Grid = [[i32; 1000]; 1000];

enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Action {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains("turn off") {
            return Ok(Action::TurnOff);
        }

        if s.contains("turn on") {
            return Ok(Action::TurnOn);
        }

        if s.contains("toggle") {
            return Ok(Action::Toggle);
        }

        Err("Could not parse action")
    }
}
