use std::cmp::min;
use utils::solution::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .split_whitespace()
                .map(as_tuple)
                .map(|[l, w, h]| 2 * (l * w + w * h + h * l) + min(min(l * w, w * h), h * l))
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            input
                .split_whitespace()
                .map(as_tuple)
                .map(|[l, w, h]| l * w * h + 2 * min(l + w, min(w + h, h + l)))
                .sum::<i32>()
                .to_string(),
        )
    }
}

fn as_tuple(s: &str) -> [i32; 3] {
    let mut it = s.split('x').map(|s| s.parse::<i32>().unwrap());
    [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
}

utils::verify!(Day02, utils::my_input!("2015", "02"), "1606483", "3842356");
