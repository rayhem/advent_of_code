use std::str::FromStr;

use itertools::Itertools;
use utils::solution::Solution;

pub struct Day11 {}

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(find_nth_good_password(input, 1).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(find_nth_good_password(input, 2).to_string())
    }
}

fn find_nth_good_password(input: &str, n: i32) -> Password {
    let mut password = Password::from_str(input).unwrap();
    for _ in 0..n {
        password.next();
    }

    password
}

#[derive(Clone, Debug)]
struct Password(Vec<u8>);

impl FromStr for Password {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.trim().chars().map(|ch| ch as u8).collect()))
    }
}

impl ToString for Password {
    fn to_string(&self) -> String {
        self.0.iter().map(|n| *n as char).join("")
    }
}

impl Password {
    fn has_run(&self) -> bool {
        self.0
            .iter()
            .tuple_windows::<(_, _, _)>()
            .any(|(a, b, c)| *c == *b + 1 && *b == *a + 1)
    }

    fn has_no_invalid_chars(&self) -> bool {
        !self
            .0
            .iter()
            .any(|&n| (n == b'i' || n == b'o' || n == b'l'))
    }

    fn has_pairs(&self) -> bool {
        let run_length_encoding = self.0.iter().fold(Vec::new(), |mut acc, el| {
            if let Some((count, value)) = acc.last_mut() {
                if *value == *el {
                    *count += 1;
                    return acc;
                }
            }

            acc.push((1, *el));
            acc
        });

        let four_in_a_row = run_length_encoding.iter().any(|(count, _)| *count > 4);
        let multiple_pairs_of_two = run_length_encoding
            .iter()
            .filter(|(count, _)| *count >= 2)
            .count()
            >= 2;

        four_in_a_row || multiple_pairs_of_two
    }

    fn is_valid(&self) -> bool {
        self.has_run() && self.has_no_invalid_chars() && self.has_pairs()
    }

    fn increment(&mut self) {
        fn next_value(n: u8) -> u8 {
            const NUM_LETTERS: u8 = b'z' - b'a' + 1;
            (n - b'a' + 1) % NUM_LETTERS + b'a'
        }

        let mut update_next_value = true;
        for i in (0..self.0.len()).rev() {
            if !update_next_value {
                break;
            }

            self.0[i] = next_value(self.0[i]);
            if self.0[i] == b'i' || self.0[i] == b'o' || self.0[i] == b'l' {
                self.0[i] = next_value(self.0[i]);
            }

            update_next_value = self.0[i] == b'a';
        }
    }

    fn next(&mut self) {
        self.increment();
        while !self.is_valid() {
            self.increment();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let password = Password::from_str("hijklmmn").unwrap();
        assert!(password.has_run());
        assert!(!password.has_no_invalid_chars());

        let password = Password::from_str("abbceffg").unwrap();
        assert!(!password.has_run());
        assert!(password.has_pairs());

        let password = Password::from_str("abbcegjk").unwrap();
        assert!(!password.has_run());
        assert!(password.has_no_invalid_chars());
        assert!(!password.has_pairs());

        let mut password = Password::from_str("abcdefgh").unwrap();
        password.next();

        assert!(password.has_run());
        assert!(password.has_no_invalid_chars());
        assert!(password.has_pairs());
        assert_eq!(password.to_string(), "abcdffaa");

        let mut password = Password::from_str("ghijklmn").unwrap();
        password.next();
        assert!(password.has_run());
        assert!(password.has_no_invalid_chars());
        assert!(password.has_pairs());
        assert_eq!(password.to_string(), "ghjaabcc");
    }
}

utils::verify!(
    Day11,
    utils::my_input!("2015", "11"),
    "hepxxyzz",
    "heqaabcc"
);
