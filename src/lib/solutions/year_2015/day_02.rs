use crate::utils::solution::Solution;
use std::cmp::min;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            total_by(input, |[l, w, h]| {
                2 * (l * w + w * h + h * l) + min(min(l * w, w * h), h * l)
            })
            .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            total_by(input, |[l, w, h]| {
                l * w * h + 2 * min(l + w, min(w + h, h + l))
            })
            .to_string(),
        )
    }
}

fn total_by(input: &str, size_fn: fn([i32; 3]) -> i32) -> i32 {
    input.split_whitespace().map(as_tuple).map(size_fn).sum()
}

fn as_tuple(s: &str) -> [i32; 3] {
    let mut it = s.split('x').map(|s| s.parse::<i32>().unwrap());
    [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
}

crate::verify!(Day02, crate::my_input!("2015", "02"), "1606483", "3842356");
