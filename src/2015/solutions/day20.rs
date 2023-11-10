use itertools::Itertools;
use utils::solution::Solution;

pub struct Day20 {}

impl Solution for Day20 {
    fn part_one(&self, input: &str) -> Option<String> {
        let n = input.trim().parse::<usize>().unwrap();
        factors(n, 10, usize::MAX).map(|house| house.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let n = input.trim().parse::<usize>().unwrap();
        factors(n, 11, 50).map(|house| house.to_string())
    }
}

fn factors(target: usize, num_presents: usize, max_steps: usize) -> Option<usize> {
    let ubound = target / num_presents;
    let mut houses = vec![0; ubound + 1];
    for factor in 1..houses.len() {
        for multiple in 1..=(ubound / factor).min(max_steps) {
            let house = factor * multiple;
            houses[house] += factor * num_presents;
        }
    }

    houses
        .iter()
        .find_position(|total| **total >= target)
        .map(|(a, _)| a)
}

utils::verify!(Day20, utils::my_input!("2015", "20"), "786240", "831600");
