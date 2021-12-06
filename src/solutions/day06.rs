use advent_utils::solution::Solution;
use std::str::FromStr;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(parse_and_count_after_days(input, 80).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(parse_and_count_after_days(input, 256).to_string())
    }
}

fn parse_and_count_after_days(s: &str, n: i32) -> i64 {
    s.parse::<FishSimulator>().unwrap().step_n(n).total()
}

#[derive(Clone, Copy, Debug, Default)]
struct FishSimulator {
    data: [i64; 9],
}

impl FishSimulator {
    fn step(&mut self) -> &Self {
        self.data.rotate_left(1);
        self.data[6] += self.data[8];

        self
    }

    fn step_n(&mut self, n: i32) -> &Self {
        (0..n).for_each(|_| {
            self.step();
        });
        self
    }

    fn total(&self) -> i64 {
        self.data.iter().sum()
    }
}

impl FromStr for FishSimulator {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s.split(',').map(|s| s.trim().parse::<i32>());
        let mut fish = Self::default();
        for digit in digits {
            fish.data[digit? as usize] += 1;
        }

        Ok(fish)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "3,4,3,1,2";

        #[test]
        fn example1() {
            assert_eq!(
                DATA.parse::<FishSimulator>().unwrap().step_n(18).total(),
                26
            );
        }

        #[test]
        fn example2() {
            assert_eq!(
                DATA.parse::<FishSimulator>().unwrap().step_n(80).total(),
                5934
            );
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day06 = Day06 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day06.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("396210")));
        }

        #[test]
        fn part_two() {
            assert_eq!(
                SOLUTION.part_two(INPUT),
                Some(String::from("1770823541496"))
            );
        }
    }
}
