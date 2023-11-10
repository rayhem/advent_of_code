use md5;

use crate::utils::solution::Solution;

pub struct Day04 {}

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(find_valid_hash(input.trim(), "00000").to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(find_valid_hash(input.trim(), "000000").to_string())
    }
}

fn find_valid_hash(input: &str, target: &str) -> i32 {
    (1..).find(|n| valid_hash(input, *n, target)).unwrap()
}

fn valid_hash(base: &str, n: i32, target: &str) -> bool {
    let s = String::from(base) + &n.to_string();
    format!("{:x}", md5::compute(s)).starts_with(target)
}

crate::verify!(Day04, crate::my_input!("2015", "04"), "346386", "9958218");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(valid_hash("abcdef", 609043, "00000"))
    }

    #[test]
    fn example2() {
        assert!(valid_hash("pqrstuv", 1048970, "00000"))
    }
}
