use crate::utils::solution::Solution;

/// Puzzle: https://adventofcode.com/2024/day/2/
pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            reports(input)
                .iter()
                .filter(|r| r.status() == Status::Safe)
                .count()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            reports(input)
                .iter()
                .filter(|r| r.status() != Status::Unsafe)
                .count()
                .to_string(),
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Status {
    Safe,
    Dampened,
    Unsafe,
}

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn status(&self) -> Status {
        fn is_safe<T: Iterator<Item = i32> + Clone>(mut values: T) -> bool {
            values.clone().all(|delta| (1..=3).contains(&delta))
                || values.all(|delta| (-3..=-1).contains(&delta))
        }

        let deltas = (0..self.levels.len()).map(|i| {
            let reduced = self
                .levels
                .iter()
                .enumerate()
                .filter(move |(n, _)| i != *n)
                .map(|(_, x)| x);

            reduced.clone().zip(reduced.skip(1)).map(|(a, b)| b - a)
        });

        if is_safe(
            self.levels
                .iter()
                .zip(self.levels.iter().skip(1))
                .map(|(a, b)| b - a),
        ) {
            Status::Safe
        } else if deltas.clone().any(is_safe) {
            Status::Dampened
        } else {
            Status::Unsafe
        }
    }
}

impl From<&str> for Report {
    fn from(value: &str) -> Self {
        Report {
            levels: value
                .split_ascii_whitespace()
                .flat_map(str::parse::<i32>)
                .collect(),
        }
    }
}

fn reports(input: &str) -> Vec<Report> {
    input.lines().map(Into::into).collect()
}

#[allow(unused_imports)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let day02 = Day02 {};
        assert_eq!(day02.part_one(INPUT), Some("2".to_string()));
        assert_eq!(day02.part_two(INPUT), Some("4".to_string()));
    }
}
