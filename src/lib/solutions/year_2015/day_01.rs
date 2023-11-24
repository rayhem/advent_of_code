use crate::utils::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .as_bytes()
                .iter()
                .map(as_number)
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            (input
                .as_bytes()
                .iter()
                .scan(0, |acc, c| {
                    *acc += as_number(c);
                    Some(*acc)
                })
                .take_while(|height| height > &-1)
                .count()
                + 1)
            .to_string(),
        )
    }
}

fn as_number(ch: &u8) -> i32 {
    match ch {
        b'(' => 1,
        b')' => -1,
        _ => 0,
    }
}

crate::verify!(Day01, crate::my_input!("2015", "01"), "280", "1797");
