use crate::solutions::{self, Solution};
use itertools::Itertools;
use solutions::AdventError;
use std::{collections::HashMap, str::FromStr};

pub struct Day14 {}

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> Option<String> {
        let mut mem = HashMap::new();
        let mut commands = input
            .lines()
            .filter_map(|line| Command::from_str(line).ok());

        let mut mask = commands.next().unwrap();
        for command in commands {
            match command {
                Command::Mask(_) => mask = command,
                Command::Memset(address, val) => {
                    if let Command::Mask(mask) = mask {
                        mem.insert(address, mask.evaluate(val));
                    }
                }
            }
        }

        Some(mem.values().sum::<u64>().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let mut mem = HashMap::new();
        let mut commands = input
            .lines()
            .filter_map(|line| Command::from_str(line).ok());

        let mut mask = commands.next().unwrap();
        for command in commands {
            match command {
                Command::Mask(_) => mask = command,
                Command::Memset(address, val) => {
                    if let Command::Mask(mask) = mask {
                        let address = (address | mask.as_1) & !mask.as_x;
                        let floating_bits: Vec<_> =
                            (0..36).filter(|b| mask.as_x & (1_u64 << b) > 0).collect();

                        for bitset in (0..=floating_bits.len())
                            .map(|len| floating_bits.iter().combinations(len))
                            .flatten()
                        {
                            mem.insert(
                                bitset.iter().fold(address, |acc, &bit| acc | (1u64 << bit)),
                                val,
                            );
                        }
                    }
                }
            }
        }

        Some(mem.values().sum::<u64>().to_string())
    }
}

#[derive(Clone, Copy, Debug)]
struct Mask {
    as_0: u64,
    as_1: u64,
    as_x: u64,
}

impl Mask {
    fn new() -> Self {
        Mask {
            as_0: 0,
            as_1: 0,
            as_x: 0,
        }
    }

    fn evaluate(&self, val: u64) -> u64 {
        (val & (!self.as_0)) | self.as_1
    }
}

impl FromStr for Mask {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const CHARS: [char; 3] = ['0', '1', 'X'];
        let mut mask = Mask::new();
        for ch in CHARS.iter() {
            match ch {
                '0' => mask.as_0 = parse_as(s, *ch),
                '1' => mask.as_1 = parse_as(s, *ch),
                'X' => mask.as_x = parse_as(s, *ch),
                _ => {}
            }
        }

        Ok(mask)
    }
}

#[derive(Clone, Copy, Debug)]
enum Command {
    Mask(Mask),
    Memset(u64, u64),
}

impl FromStr for Command {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let trigits = s.split(" = ").last().unwrap();
            Ok(Command::Mask(Mask::from_str(trigits).unwrap()))
        } else {
            let hunks: Vec<_> = s.split(|c| c == '[' || c == ']' || c == ' ').collect();
            let address = hunks[1].parse()?;
            let value = hunks[4].parse()?;

            Ok(Command::Memset(address, value))
        }
    }
}

fn parse_as(s: &str, character: char) -> u64 {
    s.chars()
        .fold(0_u64, |acc, ch| 2 * acc + ((ch == character) as u64))
}
