use crate::utils::solution::Solution;
use std::collections::{BinaryHeap, HashMap};

/// Puzzle: https://adventofcode.com/2024/day/1
pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (left, right) = sorted_lists(input);
        Some(
            left.iter()
                .zip(right.iter())
                .map(|(a, b)| (b - a).abs())
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let (left, right) = sorted_lists(input);

        let counts = right
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut counts, &value| {
                *counts.entry(value).or_insert(0) += 1;
                counts
            });

        Some(
            left.iter()
                .map(|value| value * counts.get(value).unwrap_or(&0))
                .sum::<i32>()
                .to_string(),
        )
    }
}

fn sorted_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right) = input.lines().fold(
        (BinaryHeap::<i32>::new(), BinaryHeap::<i32>::new()),
        |(mut left, mut right), line| {
            let (a, b) = line.split_once(" ").expect("Unable to split line at \" \"");
            left.push(a.trim().parse::<i32>().expect("Bad integer"));
            right.push(b.trim().parse::<i32>().expect("Bad integer"));

            (left, right)
        },
    );

    (left.into_sorted_vec(), right.into_sorted_vec())
}

mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

        let soln = Day01 {};

        assert_eq!(soln.part_one(INPUT), Some("11".to_string()));
        assert_eq!(soln.part_two(INPUT), Some("31".to_string()));
    }
}

crate::verify!(
    Day01,
    crate::my_input!("2024", "Day01"),
    "1830467",
    "26674158"
);
