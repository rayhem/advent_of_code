use std::str::FromStr;

use itertools::Itertools;
use utils::solution::Solution;

pub struct Day19 {}

impl Solution for Day19 {
    fn part_one(&self, input: &str) -> Option<String> {
        let mut lines = input.lines().collect_vec();
        let molecule = lines.pop().unwrap().to_string();
        let replacements = Replacements::from_str(&lines.iter().join("\n")).unwrap();

        Some(replacements.replace_once(&molecule).len().to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

#[derive(Clone, Debug)]
struct Replacements(Vec<(String, String)>);

impl Replacements {
    fn replace_once(&self, molecule: &str) -> Vec<String> {
        self.0
            .iter()
            .flat_map(|(term, replacement)| {
                molecule.match_indices(term).map(|(idx, _)| {
                    let (a, b) = molecule.split_at(idx);
                    let (_, c) = b.split_at(term.len());
                    a.to_owned() + replacement + c
                })
            })
            .unique()
            .collect()
    }
}

impl FromStr for Replacements {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|line| {
                    let mut tokens = line.split(" => ").map(str::to_string);
                    (tokens.next().unwrap(), tokens.next().unwrap())
                })
                .collect(),
        ))
    }
}
