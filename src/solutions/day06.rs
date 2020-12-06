use std::collections::HashSet;

use crate::solutions::Solution;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .split("\n\n")
                .map(|group| {
                    group
                        .lines()
                        .map(|line| line.chars().collect::<HashSet<char>>())
                        .fold(HashSet::new(), |acc, x| acc.union(&x).map(|c| *c).collect())
                        .len()
                })
                .sum::<usize>()
                .to_string(),
        )
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}
