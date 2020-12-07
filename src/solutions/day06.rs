use std::collections::HashSet;

use crate::solutions::Solution;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            num_questions(
                input,
                (HashSet::new(), |acc, x| acc.union(&x).map(|c| *c).collect()),
            )
            .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            num_questions(
                input,
                (('a'..='z').collect(), |acc, x| {
                    acc.intersection(&x).map(|c| *c).collect()
                }),
            )
            .to_string(),
        )
    }
}

fn num_questions(
    input: &str,
    (base, accumulator): (
        HashSet<char>,
        fn(HashSet<char>, HashSet<char>) -> HashSet<char>,
    ),
) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .fold(base.clone(), |acc, x| accumulator(acc, x))
                .len()
        })
        .sum::<usize>()
}
