use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use advent_utils::solution::Solution;

pub struct Day09 {}

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> Option<String> {
        match distance_bounds(input) {
            itertools::MinMaxResult::MinMax(a, _) => Some(a.to_string()),
            _ => panic!(),
        }
    }

    fn part_two(&self, input: &str) -> Option<String> {
        match distance_bounds(input) {
            itertools::MinMaxResult::MinMax(_, b) => Some(b.to_string()),
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct LocationPair<'a> {
    from: &'a str,
    to: &'a str,
    distance: i32,
}

impl<'a> TryFrom<&'a str> for LocationPair<'a> {
    type Error = Box<dyn std::error::Error>;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut tokens = s.split_ascii_whitespace();

        Ok(LocationPair {
            from: tokens.next().ok_or("Length error")?,
            to: tokens.nth(1).ok_or("Length error")?,
            distance: tokens.nth(1).ok_or("length error")?.parse::<i32>()?,
        })
    }
}

fn distance_bounds(input: &str) -> itertools::MinMaxResult<i32> {
    let mut places: HashSet<&str> = HashSet::new();
    let mut distances: HashMap<&str, HashMap<&str, i32>> = HashMap::new();

    for LocationPair { from, to, distance } in input.lines().map(LocationPair::try_from).flatten() {
        places.insert(from);
        places.insert(to);

        distances.entry(from).or_default().insert(to, distance);
        distances.entry(to).or_default().insert(from, distance);
    }

    places
        .iter()
        .permutations(places.len())
        .map(|v| {
            v.iter()
                .tuple_windows::<(_, _)>()
                .map(|(&&a, &&b)| distances.get(a).unwrap().get(b).unwrap())
                .sum::<i32>()
        })
        .minmax()
}

#[cfg(test)]
mod tests {
    mod part1 {
        use super::super::*;

        #[test]
        fn example0() {}
    }

    mod part2 {
        use super::super::*;

        #[test]
        fn example0() {}
    }
}
