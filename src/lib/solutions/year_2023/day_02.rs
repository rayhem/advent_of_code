use std::str::FromStr;

use crate::utils::solution::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        const TARGET: Record = Record::new(12, 13, 14);
        Some(
            input
                .lines()
                .filter_map(|line| Game::from_str(line).ok())
                .filter(|game| game.is_possible(&TARGET))
                .map(|game| game.index)
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .filter_map(|line| Game::from_str(line).ok())
                .map(|game| game.fewest_cubes())
                .map(|result| result.power())
                .sum::<i32>()
                .to_string(),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Record {
    red: i32,
    green: i32,
    blue: i32,
}

impl Record {
    const fn new(red: i32, green: i32, blue: i32) -> Self {
        Self { red, green, blue }
    }

    fn is_possible(&self, other: &Record) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Record {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut record = Record::default();

        for token in s.split(',').map(str::trim) {
            let (quantity, color) = token.split_once(' ').ok_or("malformed token")?;
            let value = quantity.trim().parse().map_err(|_| "bad quantity")?;

            match color {
                "red" => record.red = value,
                "green" => record.green = value,
                "blue" => record.blue = value,
                _ => return Err("bad color"),
            }
        }

        Ok(record)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Game {
    index: i32,
    records: Vec<Record>,
}

impl Game {
    fn is_possible(&self, record: &Record) -> bool {
        self.records.iter().all(|r| r.is_possible(record))
    }

    fn fewest_cubes(&self) -> Record {
        let mut result = Record::default();

        for record in self.records.iter() {
            result.red = result.red.max(record.red);
            result.green = result.green.max(record.green);
            result.blue = result.blue.max(record.blue);
        }

        result
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(':').ok_or("malformed string")?;

        let index = a
            .split_whitespace()
            .nth(1)
            .ok_or("malformed head")?
            .parse::<i32>()
            .map_err(|_| "bad index")?;

        let records = b
            .split(';')
            .filter_map(|r| Record::from_str(r).ok())
            .collect();

        Ok(Game { index, records })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

        let games = INPUT
            .lines()
            .filter_map(|s| Game::from_str(s).ok())
            .collect::<Vec<_>>();

        const TARGET: Record = Record::new(12, 13, 14);

        assert!(games[0].is_possible(&TARGET));
        assert!(games[1].is_possible(&TARGET));
        assert!(!games[2].is_possible(&TARGET));
        assert!(!games[3].is_possible(&TARGET));
        assert!(games[4].is_possible(&TARGET));
    }
}

crate::verify!(Day02, crate::my_input!("2023", "02"), "", "");
