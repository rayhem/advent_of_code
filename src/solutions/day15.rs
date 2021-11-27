use std::{cmp::max, str::FromStr};

use advent_utils::solution::Solution;

pub struct Day15 {}

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> Option<String> {
        let ingredients = ingredients(input);

        let mut max_score = 0;
        for i in 0..=100 {
            for j in 0..=100 {
                for k in 0..=(101 - i - j) {
                    let coef = [i, j, k, 100 - (i + j + k)];
                    let pairs = coef.iter().zip(ingredients.iter());

                    max_score = max(
                        max_score,
                        max(0, pairs.clone().map(|(a, b)| a * b.capacity).sum())
                            * max(0, pairs.clone().map(|(a, b)| a * b.durability).sum())
                            * max(0, pairs.clone().map(|(a, b)| a * b.flavor).sum())
                            * max(0, pairs.map(|(a, b)| a * b.texture).sum()),
                    );
                }
            }
        }

        Some(max_score.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let ingredients = ingredients(input);

        let mut max_score = 0;
        for i in 0..=100 {
            for j in 0..=100 {
                for k in 0..=(101 - i - j) {
                    let coef = [i, j, k, 100 - (i + j + k)];
                    let pairs = coef.iter().zip(ingredients.iter());

                    if pairs.clone().map(|(a, b)| a * b.calories).sum::<i32>() == 500 {
                        max_score = max(
                            max_score,
                            max(0, pairs.clone().map(|(a, b)| a * b.capacity).sum())
                                * max(0, pairs.clone().map(|(a, b)| a * b.durability).sum())
                                * max(0, pairs.clone().map(|(a, b)| a * b.flavor).sum())
                                * max(0, pairs.map(|(a, b)| a * b.texture).sum()),
                        );
                    }
                }
            }
        }

        Some(max_score.to_string())
    }
}

#[derive(Clone, Copy, Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_ascii_whitespace();
        Ok(Ingredient {
            capacity: tokens.nth(1).unwrap().trim_matches(',').parse()?,
            durability: tokens.nth(1).unwrap().trim_matches(',').parse()?,
            flavor: tokens.nth(1).unwrap().trim_matches(',').parse()?,
            texture: tokens.nth(1).unwrap().trim_matches(',').parse()?,
            calories: tokens.nth(1).unwrap().trim_matches(',').parse()?,
        })
    }
}

fn ingredients(input: &str) -> Vec<Ingredient> {
    input
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(_, properties)| Ingredient::from_str(properties).unwrap())
        .collect()
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
