use advent_utils::solution::Solution;
use itertools::Itertools;

pub struct Day07 {}

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> Option<String> {
        let crab_positions = crab_positions(input.trim());
        let median = *crab_positions.get(crab_positions.len() / 2).unwrap();
        Some(linear_fuel_cost(crab_positions.iter(), median).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let crab_positions = crab_positions(input.trim());
        let mean_bounds = (0..=1)
            .map(|i| (crab_positions.iter().sum::<i32>() + i) / (crab_positions.len() as i32));

        Some(
            mean_bounds
                .map(|m| quadratic_fuel_cost(crab_positions.iter(), m))
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

fn linear_fuel_cost<'a, T>(crab_positions: T, target_position: i32) -> i32
where
    T: Iterator<Item = &'a i32>,
{
    crab_positions
        .map(|position| (position - target_position).abs())
        .sum()
}

fn quadratic_fuel_cost<'a, T>(crab_positions: T, target_position: i32) -> i32
where
    T: Iterator<Item = &'a i32>,
{
    crab_positions
        .map(|position| {
            let n = (position - target_position).abs();
            n * (n + 1) / 2
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "16,1,2,0,4,2,7,1,2,14";

        #[test]
        fn example1() {
            let crab_positions = crab_positions(DATA);
            assert_eq!(linear_fuel_cost(crab_positions.iter(), 1), 41);
            assert_eq!(linear_fuel_cost(crab_positions.iter(), 2), 37); // minimal cost
            assert_eq!(linear_fuel_cost(crab_positions.iter(), 3), 39);
            assert_eq!(linear_fuel_cost(crab_positions.iter(), 10), 71);
        }

        #[test]
        fn example2() {
            let crab_positions = crab_positions(DATA);
            assert_eq!(quadratic_fuel_cost(crab_positions.iter(), 5), 168);
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day07 = Day07 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day07.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("333755")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("94017638")));
        }
    }
}
