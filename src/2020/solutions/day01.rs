use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        let expenses = parse_as_set(input);

        for entry in expenses.iter() {
            let target = 2020 - entry;
            match expenses.get(&target) {
                Some(_) => return Some((entry * target).to_string()),
                None => continue,
            }
        }

        None
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let expenses = parse_as_set(input);

        for pair in expenses.iter().combinations(2) {
            let target = 2020 - (pair[0] + pair[1]);
            match expenses.get(&target) {
                Some(_) => return Some((pair[0] * pair[1] * target).to_string()),
                None => continue,
            }
        }

        None
    }
}

fn parse_as_set(input: &str) -> HashSet<i32> {
    input
        .split_ascii_whitespace()
        .map(|t| t.parse::<i32>().unwrap())
        .collect()
}
