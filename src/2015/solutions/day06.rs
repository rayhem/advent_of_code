use glam::IVec2;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::str::FromStr;

use utils::solution::Solution;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> Option<String> {
        let rule = |action: &Action, x: LightStatus| match action {
            Action::Toggle => 1 - x,
            Action::TurnOff => max(0, x - 1),
            Action::TurnOn => min(1, x + 1),
        };

        Some(total_lights(input, rule).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let rule = |action: &Action, x: LightStatus| match action {
            Action::Toggle => x + 2,
            Action::TurnOff => max(0, x - 1),
            Action::TurnOn => x + 1,
        };

        Some(total_lights(input, rule).to_string())
    }
}

fn total_lights(input: &str, update_rule: fn(&Action, LightStatus) -> LightStatus) -> LightStatus {
    let mut grid: std::boxed::Box<Grid> = std::boxed::Box::new([[0; 1000]; 1000]);

    for Instruction {
        action,
        lbound,
        ubound,
    } in input.lines().flat_map(Instruction::from_str)
    {
        for row in lbound.y..=ubound.y {
            for col in lbound.x..=ubound.x {
                grid[row as usize][col as usize] =
                    update_rule(&action, grid[row as usize][col as usize]);
            }
        }
    }

    grid.iter().flatten().sum()
}

type LightStatus = i32;
type Grid = [[LightStatus; 1000]; 1000];

enum Action {
    Toggle,
    TurnOff,
    TurnOn,
}

struct Instruction {
    action: Action,
    lbound: IVec2,
    ubound: IVec2,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(\\d+),(\\d+) through (\\d+),(\\d+)").unwrap();
        }

        let captures = RE.captures(s).ok_or("Could not parse coordinates")?;
        let coordinates = (1..=4)
            .map(|i| captures[i].parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let lbound = IVec2::new(coordinates[0], coordinates[1]);
        let ubound = IVec2::new(coordinates[2], coordinates[3]);

        Ok(Instruction {
            action: if s.contains("toggle") {
                Action::Toggle
            } else if s.contains("turn off") {
                Action::TurnOff
            } else if s.contains("turn on") {
                Action::TurnOn
            } else {
                return Err("Could not evaluate action");
            },
            lbound,
            ubound,
        })
    }
}

utils::verify!(Day06, utils::my_input!("2015", "06"), "400410", "15343601");
