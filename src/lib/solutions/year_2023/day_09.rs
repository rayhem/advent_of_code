use std::{num::ParseIntError, str::FromStr};

use itertools::Itertools;

use crate::utils::solution::Solution;

pub struct Day09 {}

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .flat_map(Sequence::from_str)
                .map(|s| s.future())
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .flat_map(Sequence::from_str)
                .map(|s| s.past())
                .sum::<i32>()
                .to_string(),
        )
    }
}

type Number = i32;

struct Sequence {
    numbers: Vec<Number>,
}

impl Sequence {
    fn derivatives(&self) -> Vec<Vec<Number>> {
        let mut derivatives = vec![self.numbers.clone()];
        while let Some(d) = derivatives.last() {
            if d.iter().all(|n| *n == 0) {
                break;
            }

            derivatives.push(d.iter().tuple_windows().map(|(a, b)| b - a).collect());
        }

        derivatives
    }

    fn future(&self) -> Number {
        self.derivatives()
            .iter()
            .map(|d| d.last().unwrap_or(&0))
            .sum::<Number>()
    }

    fn past(&self) -> Number {
        self.derivatives()
            .iter()
            .rev()
            .fold(0, |value, derivative| {
                -(value - derivative.first().unwrap())
            })
    }
}

impl FromStr for Sequence {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<Number>, ParseIntError> =
            s.split_whitespace().map(str::parse::<Number>).collect();
        Ok(Self {
            numbers: values.map_err(|_| "Failed to parse values")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        let result: Result<Vec<_>, _> = INPUT.lines().map(Sequence::from_str).collect();
        let sequences = result.expect("Failed to parse input");

        assert_eq!(sequences[0].future(), 18);
        assert_eq!(sequences[1].future(), 28);
        assert_eq!(sequences[2].future(), 68);

        assert_eq!(sequences[0].past(), -3);
        assert_eq!(sequences[1].past(), 0);
        assert_eq!(sequences[2].past(), 5);
    }
}

crate::verify!(
    Day09,
    crate::my_input!("2023", "Day09"),
    "2105961943",
    "1019"
);
