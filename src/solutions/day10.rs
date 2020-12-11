use std::collections::HashMap;

use crate::solutions::Solution;
use itertools::Itertools;

pub struct Day10 {}

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> Option<String> {
        let mut ones_count = 0;
        let mut threes_count = 0;
        for d in std::iter::once(&0)
            .chain(parse_input(input).iter())
            .tuple_windows()
            .map(|(a, b)| b - a)
            .chain(std::iter::once(3))
        {
            match d {
                1 => ones_count += 1,
                3 => threes_count += 1,
                _ => {}
            }
        }

        Some((ones_count * threes_count).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let jolts = parse_input(input);
        Some(count_paths(&jolts).values().max().unwrap().to_string())
    }
}

fn count_paths(joltages: &Vec<i32>) -> HashMap<i32, i64> {
    let mut paths = HashMap::new();
    paths.insert(0, 1);

    for joltage in joltages {
        let num_paths_to = |val| paths.get(&val).unwrap_or(&0);
        let num_new_paths =
            num_paths_to(joltage - 3) + num_paths_to(joltage - 2) + num_paths_to(joltage - 1);
        paths.insert(*joltage, num_new_paths);
    }

    paths
}

fn parse_input(input: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    numbers.sort();

    numbers
}
