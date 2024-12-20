use regex::Regex;
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

use crate::utils::solution::Solution;

pub struct Day04 {}

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(check_passports(input, |passport| passport.is_valid()).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(check_passports(input, |passport| passport.is_really_valid()).to_string())
    }
}

fn check_passports(input: &str, checker: fn(&Passport) -> bool) -> usize {
    input
        .split("\n\n")
        .filter(|passport| checker(&Passport::from_str(passport).unwrap()))
        .count()
}

#[derive(Clone, Debug)]
enum Day04Error {
    ParseIntError,
    UnknownField,
}

impl From<std::num::ParseIntError> for Day04Error {
    fn from(_err: ParseIntError) -> Day04Error {
        Day04Error::ParseIntError
    }
}

#[derive(Clone, Copy, Debug)]
enum Height {
    Metric(i32),
    Imperial(i32),
    Unknown,
}

impl FromStr for Height {
    type Err = Day04Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (i, _) = s.char_indices().rev().nth(1).unwrap();
        Ok(match &s[i..] {
            "cm" => Height::Metric(s[..i].parse::<i32>()?),
            "in" => Height::Imperial(s[..i].parse::<i32>()?),
            _ => Height::Unknown,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
    Unknown,
}

impl FromStr for EyeColor {
    type Err = Day04Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" => Ok(EyeColor::Amber),
            "blu" => Ok(EyeColor::Blue),
            "brn" => Ok(EyeColor::Brown),
            "gry" => Ok(EyeColor::Grey),
            "grn" => Ok(EyeColor::Green),
            "hzl" => Ok(EyeColor::Hazel),
            "oth" => Ok(EyeColor::Other),
            _ => Ok(EyeColor::Unknown),
        }
    }
}
#[derive(Clone, Debug, Default)]
struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<EyeColor>,
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

    fn is_really_valid(&self) -> bool {
        let hair_regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
        let conditions: Vec<Option<bool>> = vec![
            self.birth_year.map(|y| (1920..=2002).contains(&y)),
            self.issue_year.map(|y| (2010..=2020).contains(&y)),
            self.expiration_year.map(|y| (2020..=2030).contains(&y)),
            self.height.map(|h| match h {
                Height::Metric(x) => (150..=193).contains(&x),
                Height::Imperial(x) => (59..=76).contains(&x),
                Height::Unknown => false,
            }),
            self.hair_color
                .as_ref()
                .map(|color| hair_regex.is_match(color)),
            self.eye_color.map(|color| color != EyeColor::Unknown),
            self.passport_id.as_ref().map(|s| s.len() == 9),
        ];

        conditions.iter().all(|c| c.unwrap_or_default())
    }
}

impl FromStr for Passport {
    type Err = Day04Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = HashMap::new();
        for token in s.split_whitespace() {
            let mut x = token.split(':');
            fields.insert(x.next().unwrap(), x.next().unwrap());
        }

        let mut passport = Passport::default();
        for (key, value) in fields.into_iter() {
            match key {
                "byr" => passport.birth_year = Some(value.parse()?),
                "iyr" => passport.issue_year = Some(value.parse()?),
                "eyr" => passport.expiration_year = Some(value.parse()?),
                "hgt" => passport.height = Some(value.parse()?),
                "hcl" => passport.hair_color = Some(value.to_string()),
                "ecl" => passport.eye_color = value.parse().ok(),
                "pid" => passport.passport_id = Some(value.to_string()),
                "cid" => passport.country_id = Some(value.to_string()),
                _ => return Err(Day04Error::UnknownField),
            }
        }

        Ok(passport)
    }
}
