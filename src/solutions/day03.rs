use crate::solutions::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(count_trees(input, (3usize, 1usize)).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        Some(
            slopes
                .into_iter()
                .map(|slope| count_trees(input, slope))
                .product::<usize>()
                .to_string(),
        )
    }
}

fn count_trees(input: &str, (dx, dy): (usize, usize)) -> usize {
    let width = input.lines().next().unwrap().chars().count();
    input
        .lines()
        .step_by(dy)
        .enumerate()
        .filter(|(i, line)| line.chars().nth((i * dx) % width) == Some('#'))
        .count()
}
