use crate::solutions::Solution;
use std::{option::Option, str::FromStr};

pub struct Day02 {}

impl Solution for Day02 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .map(as_pair)
                .filter(|(rule, password)| rule.sled_validate(password))
                .count()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .map(as_pair)
                .filter(|(rule, password)| rule.toboggan_validate(password))
                .count()
                .to_string(),
        )
    }
}

fn as_pair(s: &str) -> (PasswordRule, &str) {
    let mut tokens = s.split(':');
    (
        PasswordRule::from_str(tokens.next().unwrap()).unwrap(),
        tokens.next().unwrap(),
    )
}

#[derive(Debug, PartialEq)]
struct PasswordRule {
    values: (i32, i32),
    letter: char,
}

impl PasswordRule {
    fn sled_validate(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| c == &self.letter).count() as i32;
        self.values.0 <= count && count <= self.values.1
    }

    fn toboggan_validate(&self, password: &str) -> bool {
        let chars = password.chars().filter(|c| !c.is_whitespace());
        let l1 = chars
            .clone()
            .nth(self.values.0 as usize - 1)
            .unwrap_or_default();
        let l2 = chars
            .clone()
            .nth(self.values.1 as usize - 1)
            .unwrap_or_default();

        (l1 == self.letter) != (l2 == self.letter)
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
            values: (
                bounds
                    .next()
                    .ok_or(PasswordRuleParseError::MalformedRange)?
                    .map_err(|_| PasswordRuleParseError::ParseIntError)?,
                bounds
                    .next()
                    .ok_or(PasswordRuleParseError::MalformedRange)?
                    .map_err(|_| PasswordRuleParseError::ParseIntError)?,
            ),
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

    fn rules() -> Vec<PasswordRule> {
        vec![
            PasswordRule {
                values: (1, 3),
                letter: 'a',
            },
            PasswordRule {
                values: (1, 3),
                letter: 'b',
            },
            PasswordRule {
                values: (2, 9),
                letter: 'c',
            },
        ]
    }

    #[test]
    fn example_sled_validation() {
        let rules = rules();
        assert_eq!(rules[0].sled_validate("abcde"), true);
        assert_eq!(rules[1].sled_validate("cdefg"), false);
        assert_eq!(rules[2].sled_validate("ccccccccc"), true);
    }

    #[test]
    fn example_toboggan_validation() {
        let rules = rules();
        assert_eq!(rules[0].toboggan_validate("abcde"), true);
        assert_eq!(rules[0].toboggan_validate("cdefg"), false);
        assert_eq!(rules[0].toboggan_validate("ccccccccc"), false);
    }

    #[test]
    fn parse_validation() {
        assert_eq!(
            PasswordRule::from_str("1-3 a").expect("Could not convert rule"),
            PasswordRule {
                values: (1, 3),
                letter: 'a',
            }
        );

        assert_eq!(
            PasswordRule::from_str("80-90 f").expect("Could not convert rule"),
            PasswordRule {
                values: (80, 90),
                letter: 'f',
            }
        );
    }
}
