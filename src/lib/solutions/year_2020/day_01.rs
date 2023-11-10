use crate::utils::solution::Solution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day01 {}

const VALUE: i32 = 2020;

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        let expenses = parse_as_set(input);

        for entry in expenses.iter() {
            let target = VALUE - entry;
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
            let target = VALUE - (pair[0] + pair[1]);
            match expenses.get(&target) {
                Some(_) => return Some((pair[0] * pair[1] * target).to_string()),
                None => continue,
            }
        }

        None
    }
}

fn parse_as_set(input: &str) -> HashSet<i32> {
    input.split_whitespace().flat_map(str::parse).collect()
}
