use std::str::FromStr;

use utils::solution::Solution;

pub struct Day16 {}

impl Solution for Day16 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            (input
                .lines()
                .map(|line| line.split_once(':').unwrap().1)
                .map(Sue::from_str)
                .flatten()
                .enumerate()
                .filter(|(_, sue)| sue.matches_ideal_sue())
                .next()
                .unwrap()
                .0
                + 1)
            .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            (input
                .lines()
                .map(|line| line.split_once(':').unwrap().1)
                .map(Sue::from_str)
                .flatten()
                .enumerate()
                .filter(|(_, sue)| sue.really_matches_ideal_sue())
                .next()
                .unwrap()
                .0
                + 1)
            .to_string(),
        )
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Sue {
    akitas: Option<i32>,
    cars: Option<i32>,
    cats: Option<i32>,
    children: Option<i32>,
    goldfish: Option<i32>,
    perfumes: Option<i32>,
    pomeranians: Option<i32>,
    samoyeds: Option<i32>,
    trees: Option<i32>,
    vizslas: Option<i32>,
}

impl FromStr for Sue {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sue = Sue::default();
        let tokens = s.split(',').map(|t| t.split_once(':').unwrap());
        for (key, value) in tokens {
            let parsed_value = value.trim().parse::<i32>()?;
            match key.trim() {
                "akitas" => sue.akitas = Some(parsed_value),
                "cars" => sue.cars = Some(parsed_value),
                "cats" => sue.cats = Some(parsed_value),
                "children" => sue.children = Some(parsed_value),
                "goldfish" => sue.goldfish = Some(parsed_value),
                "perfumes" => sue.perfumes = Some(parsed_value),
                "pomeranians" => sue.pomeranians = Some(parsed_value),
                "samoyeds" => sue.samoyeds = Some(parsed_value),
                "trees" => sue.trees = Some(parsed_value),
                "vizslas" => sue.vizslas = Some(parsed_value),
                _ => continue,
            }
        }

        Ok(sue)
    }
}

impl Sue {
    fn matches_ideal_sue(&self) -> bool {
        (self.akitas.is_none() || self.akitas.unwrap() == 0)
            && (self.cars.is_none() || self.cars.unwrap() == 2)
            && (self.cats.is_none() || self.cats.unwrap() == 7)
            && (self.children.is_none() || self.children.unwrap() == 3)
            && (self.goldfish.is_none() || self.goldfish.unwrap() == 5)
            && (self.perfumes.is_none() || self.perfumes.unwrap() == 1)
            && (self.pomeranians.is_none() || self.pomeranians.unwrap() == 3)
            && (self.samoyeds.is_none() || self.samoyeds.unwrap() == 2)
            && (self.trees.is_none() || self.trees.unwrap() == 3)
            && (self.vizslas.is_none() || self.vizslas.unwrap() == 0)
    }

    fn really_matches_ideal_sue(&self) -> bool {
        (self.akitas.is_none() || self.akitas.unwrap() == 0)
            && (self.cars.is_none() || self.cars.unwrap() == 2)
            && (self.cats.is_none() || self.cats.unwrap() > 7)
            && (self.children.is_none() || self.children.unwrap() == 3)
            && (self.goldfish.is_none() || self.goldfish.unwrap() < 5)
            && (self.perfumes.is_none() || self.perfumes.unwrap() == 1)
            && (self.pomeranians.is_none() || self.pomeranians.unwrap() < 3)
            && (self.samoyeds.is_none() || self.samoyeds.unwrap() == 2)
            && (self.trees.is_none() || self.trees.unwrap() > 3)
            && (self.vizslas.is_none() || self.vizslas.unwrap() == 0)
    }
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
