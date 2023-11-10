use utils::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(input.chars().map(as_number).sum::<i32>().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            (input
                .chars()
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

fn as_number(ch: char) -> i32 {
    match ch {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

utils::verify!(Day01, utils::my_input!("2015", "01"), "280", "1797");
