use itertools::Itertools;
use utils::solution::Solution;

pub struct Day17 {}

const MIN_COMBINATIONS: usize = 4;
const TARGET_VOLUME: i32 = 150;

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> Option<String> {
        let containers = container_sizes(input);

        let total = (MIN_COMBINATIONS..containers.len()).fold(0, |acc, n| {
            acc + containers
                .iter()
                .combinations(n)
                .filter(|subset| subset.iter().fold(0, |acc, item| acc + **item) == TARGET_VOLUME)
                .count()
        });

        Some(total.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let containers = container_sizes(input);

        let total = containers
            .iter()
            .combinations(MIN_COMBINATIONS)
            .filter(|subset| subset.iter().fold(0, |acc, n| acc + **n) == 150)
            .count();

        Some(total.to_string())
    }
}

fn container_sizes(s: &str) -> Vec<i32> {
    s.lines().flat_map(str::parse::<i32>).collect()
}

utils::verify!(Day17, utils::my_input!("2015", "17"), "654", "57");
