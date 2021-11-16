use crate::solutions::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .chars()
                .fold(0, |total, ch| total + as_number(ch))
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let mut total = 0;
        for (i, ch) in input.chars().enumerate() {
            total += as_number(ch);
            if total == -1 {
                return Some((i + 1).to_string());
            }
        }

        None
    }
}

fn as_number(ch: char) -> i32 {
    match ch {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}
