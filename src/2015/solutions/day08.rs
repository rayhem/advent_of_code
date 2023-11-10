use utils::solution::Solution;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .map(length_difference)
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .map(encoded_difference)
                .sum::<i32>()
                .to_string(),
        )
    }
}

fn length_difference(s: &str) -> i32 {
    let mut chars = s.chars();
    let mut sum = 0;

    while let Some(ch) = chars.next() {
        sum += match ch {
            '\\' => match chars.next().unwrap() {
                '\\' => 1,
                '"' => 1,
                'x' => {
                    chars.next();
                    chars.next();
                    3
                }
                _ => panic!("Invalid escape sequence"),
            },
            _ => 0,
        }
    }

    sum + 2
}

fn encoded_difference(s: &str) -> i32 {
    s.chars()
        .map(|ch| match ch {
            '\\' => 2,
            '"' => 2,
            _ => 1,
        })
        .sum::<i32>()
        + 2
        - s.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_lengths() {
        assert_eq!(length_difference(&r#""#), 2);
        assert_eq!(length_difference(&r#"abc"#), 2);
        assert_eq!(length_difference(&r#"aaa\"aaa"#), 3);
        assert_eq!(length_difference(&r#"\x27"#), 5);
    }

    #[test]
    fn reencoding() {
        assert_eq!(encoded_difference(r#""""#), 4);
        assert_eq!(encoded_difference(r#""abc""#), 4);
        assert_eq!(encoded_difference(r#"aaa\"aaa"#), 4);
        assert_eq!(encoded_difference(r#"\x27"#), 3);
    }
}

utils::verify!(Day08, utils::my_input!("2015", "08"), "1371", "2117");
