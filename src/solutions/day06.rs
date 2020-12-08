use std::collections::HashSet;

use crate::solutions::Solution;

type AnswerSet = HashSet<char>;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            num_questions(
                input,
                (HashSet::new(), |acc, x| acc.union(&x).copied().collect()),
            )
            .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            num_questions(
                input,
                (('a'..='z').collect(), |acc, x| {
                    acc.intersection(&x).copied().collect()
                }),
            )
            .to_string(),
        )
    }
}

type SetAccumulation = (AnswerSet, fn(AnswerSet, AnswerSet) -> AnswerSet);

fn num_questions(input: &str, (base, accumulator): SetAccumulation) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<AnswerSet>())
                .fold(base.clone(), accumulator)
                .len()
        })
        .sum::<usize>()
}
