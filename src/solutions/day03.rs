use crate::solutions::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> Option<String> {
        let width = input.lines().next().unwrap().chars().count();

        Some(
            input
                .lines()
                .enumerate()
                .filter(|(i, line)| line.chars().nth((i * 3) % width) == Some('#'))
                .count()
                .to_string(),
        )
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}
