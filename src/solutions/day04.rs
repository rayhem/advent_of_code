use md5;

use advent_utils::solution::Solution;

pub struct Day04 {}

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(find_valid_hash(input, "00000").to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(find_valid_hash(input, "000000").to_string())
    }
}

fn find_valid_hash(input: &str, target: &str) -> i32 {
    let mut i = 0;
    while !valid_hash(input, i, target) {
        i += 1;
    }

    i
}

fn valid_hash(base: &str, n: i32, target: &str) -> bool {
    let s = String::from(base) + &n.to_string();
    format!("{:?}", md5::compute(s)).starts_with(target)
}
