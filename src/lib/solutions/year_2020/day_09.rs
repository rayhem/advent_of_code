use crate::utils::solution::Solution;
use itertools::Itertools;
use std::collections::HashSet;

type Numbers = Vec<i64>;

pub struct Day09 {}

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> Option<String> {
        incorrect_number(&parse_input(input), 25).map(|v| v.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let numbers = parse_input(input);
        let target_number = incorrect_number(&numbers, 25)?;

        let mut head = 0;
        let mut tail = 0;
        let mut range_sum = 0;

        while range_sum != target_number {
            match range_sum.cmp(&target_number) {
                std::cmp::Ordering::Less => {
                    tail += 1;
                    range_sum += numbers[tail];
                }
                std::cmp::Ordering::Greater => {
                    head += 1;
                    range_sum -= numbers[head];
                }
                std::cmp::Ordering::Equal => {}
            }
        }

        let range = &numbers[head..=tail];
        Some((range.iter().min().unwrap() + range.iter().max().unwrap()).to_string())
    }
}

fn parse_input(input: &str) -> Numbers {
    input.lines().map(|n| n.parse::<i64>().unwrap()).collect()
}

fn incorrect_number(numbers: &[i64], window_size: usize) -> Option<i64> {
    let mut sums = HashSet::new();
    numbers
        .iter()
        .skip(window_size)
        .zip(numbers.windows(window_size))
        .find(|(n, window)| {
            sums.clear();
            sums.extend(window.iter().tuple_combinations().map(|(a, b)| a + b));
            !sums.contains(n)
        })
        .map(|(a, _)| *a)
}
