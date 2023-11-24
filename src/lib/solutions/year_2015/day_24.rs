use crate::utils::solution::Solution;
use itertools::{self, Itertools};

pub struct Day24 {}

impl Solution for Day24 {
    fn part_one(&self, input: &str) -> Option<String> {
        let weights = weights(input);
        let target_weight = weights.iter().sum::<isize>() / 3;

        target_entanglement(weights, target_weight / 4).map(|v| v.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let weights = weights(input);
        let target_weight = weights.iter().sum::<isize>() / 4;

        target_entanglement(weights, target_weight / 4).map(|v| v.to_string())
    }
}

fn weights(input: &str) -> Vec<isize> {
    input.lines().flat_map(|l| l.parse()).collect()
}

fn target_entanglement(weights: Vec<isize>, target: isize) -> Option<isize> {
    (0..weights.len())
        .flat_map(|i| weights.clone().into_iter().combinations(i))
        .find(|subset| subset.iter().sum::<isize>() == target)
        .map(|subset| subset.iter().product::<isize>())
}

crate::verify!(
    Day24,
    crate::my_input!("2015", "24"),
    "10439961859",
    "72050269"
);
