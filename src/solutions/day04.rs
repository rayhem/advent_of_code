use std::str::FromStr;

use crate::solutions::Solution;

pub struct Day04 {}

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .split("\n\n")
                .filter(|passport| Passport::from_str(passport).unwrap().is_valid())
                .count()
                .to_string(),
        )
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

#[derive(Clone, Debug, Default)]
struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }
}

#[derive(Clone, Copy, Debug)]
enum ParseError {
    ParseIntError,
}

impl FromStr for Passport {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport: Passport = Default::default();
        for token in s.trim().split_whitespace() {
            match token.split_at(4) {
                ("byr:", value) => passport.birth_year = Some(value.parse::<i32>().unwrap()),
                ("iyr:", value) => passport.issue_year = Some(value.parse::<i32>().unwrap()),
                ("eyr:", value) => passport.expiration_year = Some(value.parse::<i32>().unwrap()),
                ("hgt:", value) => passport.height = Some(value.to_string()),
                ("hcl:", value) => passport.hair_color = Some(value.to_string()),
                ("ecl:", value) => passport.eye_color = Some(value.to_string()),
                ("pid:", value) => passport.passport_id = Some(value.to_string()),
                ("cid:", value) => passport.country_id = Some(value.to_string()),
                _ => (),
            }
        }

        Ok(passport)
    }
}
