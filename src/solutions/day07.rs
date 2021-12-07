use advent_utils::solution::Solution;
use itertools::Itertools;

pub struct Day07 {}

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> Option<String> {
        let crab_positions = crab_positions(input.trim());
        let median = *crab_positions.get(crab_positions.len() / 2).unwrap();
        Some(linear_fuel_cost(crab_positions.into_iter(), median).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let crab_positions = crab_positions(input.trim()).into_iter();

        let mean_bounds = (0..=1)
            .map(|i| (crab_positions.clone().sum::<i32>() + i) / (crab_positions.len() as i32));

        Some(
            mean_bounds
                .map(|m| quadratic_fuel_cost(crab_positions.clone(), m))
                .min()
                .unwrap()
                .to_string(),
        )
    }
}

fn crab_positions(s: &str) -> Vec<i32> {
    s.split(',')
        .map(|s| s.parse().unwrap())
        .sorted_unstable()
        .collect()
}

fn linear_fuel_cost<T>(crab_positions: T, target_position: i32) -> i32
where
    T: Iterator<Item = i32>,
{
    crab_positions
        .map(|position| (position - target_position).abs())
        .sum()
}

fn quadratic_fuel_cost<T>(crab_positions: T, target_position: i32) -> i32
where
    T: Iterator<Item = i32>,
{
    crab_positions
        .map(|position| {
            let n = (position - target_position).abs();
            n * (n + 1) / 2
        })
        .sum()
}
