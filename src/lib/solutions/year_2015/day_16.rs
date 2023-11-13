use crate::utils::solution::Solution;
use itertools::Itertools;
use std::{collections::HashMap, str::FromStr};

pub struct Day16 {}

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> Option<String> {
        find_sue_by(input, Sue::is_valid_part_1).map(|x| (x + 1).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        find_sue_by(input, Sue::is_valid_part_2).map(|x| (x + 1).to_string())
    }
}

fn find_sue_by(s: &str, comparison: fn(&Sue, &HashMap<String, i32>) -> bool) -> Option<usize> {
    let sues: Vec<_> = s
        .lines()
        .filter_map(|l| Sue::from_str(l.split_once(':').unwrap().1).ok())
        .collect();

    let tape: HashMap<String, i32> = [
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]
    .into_iter()
    .collect();

    sues.iter()
        .find_position(|sue| comparison(sue, &tape))
        .map(|(i, _)| i)
}

#[derive(Clone, Debug, Default)]
struct Sue(HashMap<String, i32>);

impl Sue {
    fn is_valid_part_1(&self, tape: &HashMap<String, i32>) -> bool {
        self.0
            .iter()
            .all(|(key, value)| tape.get(key) == Some(value))
    }

    fn is_valid_part_2(&self, tape: &HashMap<String, i32>) -> bool {
        self.0.iter().all(|(key, value)| match key.as_str() {
            "cats" | "trees" => tape
                .get(key)
                .map_or(false, |&tape_value| *value > tape_value),
            "pomeranians" | "goldfish" => tape
                .get(key)
                .map_or(false, |&tape_value| *value < tape_value),
            _ => tape.get(key).to_owned() == Some(&value),
        })
    }
}

impl FromStr for Sue {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields = s.split(',').filter_map(|token| {
            token
                .split_once(':')
                .map(|(a, b)| (a.trim().to_string(), b.trim().parse::<i32>().unwrap()))
        });

        Ok(Self(fields.collect()))
    }
}

crate::verify!(Day16, crate::my_input!("2015", "16"), "213", "323");
