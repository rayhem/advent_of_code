use crate::solutions::Solution;
use std::str::FromStr;

use super::AdventError;

pub struct Day16 {}

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (constraints, _, tickets) = parse_input(input).unwrap();

        Some(
            tickets
                .iter()
                .map(|ticket| ticket.error_rate(&constraints))
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn parse_input(input: &str) -> Result<(Vec<Constraint>, Ticket, Vec<Ticket>), AdventError> {
    let mut parts = input.split("\n\n");

    let constraints = parts
        .next()
        .ok_or(AdventError::BadInput)?
        .trim()
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let my_ticket = parts
        .next()
        .ok_or(AdventError::BadInput)?
        .trim()
        .lines()
        .last()
        .unwrap()
        .parse()?;

    let tickets = parts
        .next()
        .ok_or(AdventError::BadInput)?
        .trim()
        .lines()
        .skip(1) // Skip "nearby tickets: "
        .filter_map(|line| line.parse().ok())
        .collect();

    Ok((constraints, my_ticket, tickets))
}

#[derive(Clone, Debug)]
struct Ticket {
    values: Vec<i32>,
}

impl Ticket {
    fn error_rate(&self, constraints: &Vec<Constraint>) -> i32 {
        self.values
            .iter()
            .filter(|&&value| {
                !constraints
                    .iter()
                    .any(|constraint| constraint.validate(value))
            })
            .sum()
    }
}

impl FromStr for Ticket {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            values: s.split(',').filter_map(|v| v.parse().ok()).collect(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Range {
    low: i32,
    high: i32,
}

impl Range {
    fn contains(&self, value: i32) -> bool {
        (self.low <= value) && (value <= self.high)
    }
}

impl FromStr for Range {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        Ok(Range {
            low: parts.next().ok_or(AdventError::BadInput)?.parse()?,
            high: parts.next().ok_or(AdventError::BadInput)?.parse()?,
        })
    }
}

#[derive(Clone, Debug)]
struct Constraint {
    name: String,
    ranges: (Range, Range),
}

impl Constraint {
    fn validate(&self, value: i32) -> bool {
        self.ranges.0.contains(value) || self.ranges.1.contains(value)
    }
}

impl FromStr for Constraint {
    type Err = AdventError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");
        let name = parts.next().ok_or(AdventError::BadInput)?.to_string();
        let mut constraint_specs = parts.next().ok_or(AdventError::BadInput)?.split(" or ");

        Ok(Constraint {
            name,
            ranges: (
                constraint_specs
                    .next()
                    .ok_or(AdventError::BadInput)?
                    .parse()?,
                constraint_specs
                    .next()
                    .ok_or(AdventError::BadInput)?
                    .parse()?,
            ),
        })
    }
}
