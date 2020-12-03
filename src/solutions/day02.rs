use crate::solutions::Solution;
use std::{option::Option, str::FromStr};

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        None
    }

    fn part_two(&self, input: &str) -> Option<String> {
        None
    }
}

#[derive(Debug, PartialEq)]
struct PasswordRule {
    lbound: i32,
    ubound: i32,
    letter: char,
}

impl PasswordRule {
    fn validate(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.letter).count() as i32;
        self.lbound <= count && count <= self.ubound
    }
}

#[derive(Clone, Copy, Debug)]
enum PasswordRuleParseError {
    CharacterUnavailable,
    MalformedRange,
    ParseIntError,
}

impl FromStr for PasswordRule {
    type Err = PasswordRuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = s.split_ascii_whitespace().collect();
        let mut bounds = tokens[0].split('-').map(|t| t.parse::<i32>());

        Ok(PasswordRule {
            lbound: bounds
                .next()
                .ok_or(PasswordRuleParseError::MalformedRange)?
                .map_err(|_| PasswordRuleParseError::ParseIntError)?,
            ubound: bounds
                .next()
                .ok_or(PasswordRuleParseError::MalformedRange)?
                .map_err(|_| PasswordRuleParseError::ParseIntError)?,
            letter: tokens[1]
                .chars()
                .next()
                .ok_or(PasswordRuleParseError::CharacterUnavailable)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_data() {
        assert_eq!(
            PasswordRule::from_str("1-3 a").unwrap().validate("abcde"),
            true
        );
        assert_eq!(
            PasswordRule::from_str("1-3 b").unwrap().validate("cdefg"),
            false
        );
        assert_eq!(
            PasswordRule::from_str("2-9 c")
                .unwrap()
                .validate("ccccccccc"),
            true
        );
    }
}
