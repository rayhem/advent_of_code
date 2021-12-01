use advent_utils::solution::Solution;
use itertools::{Itertools, TupleWindows};

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(get_positive_deltas(input.lines().map(|l| l.parse::<i32>().unwrap())).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            get_positive_deltas(
                input
                    .lines()
                    .map(|l| l.parse::<i32>().unwrap())
                    .tuple_windows::<(_, _, _)>()
                    .map(|(a, b, c)| a + b + c),
            )
            .to_string(),
        )
    }
}

fn get_positive_deltas<T>(xs: T) -> i32
where
    T: Iterator<Item = i32>,
{
    xs.tuple_windows().filter(|(a, b)| b > a).count() as i32
}

#[cfg(test)]
mod test {
    use super::*;
    static DATA: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn positive_deltas() {
        assert_eq!(get_positive_deltas(DATA.into_iter()), 7)
    }
}
