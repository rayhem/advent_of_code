use std::{cmp::max, collections::HashMap, str::FromStr};

use advent_utils::solution::Solution;

pub struct Day15 {}

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> Option<String> {
        let ingredients = ingredients(input);

        let mut max_score = 0;
        for i in 0..=100 {
            for j in 0..=100 {
                for k in 0..=(101 - i - j) {
                    let l = 100 - (i + j + k);

                    max_score = max(
                        max_score,
                        max(
                            0,
                            i * ingredients[0].capacity
                                + j * ingredients[1].capacity
                                + k * ingredients[2].capacity
                                + l * ingredients[3].capacity,
                        ) * max(
                            0,
                            i * ingredients[0].durability
                                + j * ingredients[1].durability
                                + k * ingredients[2].durability
                                + l * ingredients[3].durability,
                        ) * max(
                            0,
                            i * ingredients[0].flavor
                                + j * ingredients[1].flavor
                                + k * ingredients[2].flavor
                                + l * ingredients[3].flavor,
                        ) * max(
                            0,
                            i * ingredients[0].texture
                                + j * ingredients[1].texture
                                + k * ingredients[2].texture
                                + l * ingredients[3].texture,
                        ),
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
                    let l = 100 - (i + j + k);

                    if i * ingredients[0].calories
                        + j * ingredients[1].calories
                        + k * ingredients[2].calories
                        + l * ingredients[3].calories
                        == 500
                    {
                        max_score = max(
                            max_score,
                            max(
                                0,
                                i * ingredients[0].capacity
                                    + j * ingredients[1].capacity
                                    + k * ingredients[2].capacity
                                    + l * ingredients[3].capacity,
                            ) * max(
                                0,
                                i * ingredients[0].durability
                                    + j * ingredients[1].durability
                                    + k * ingredients[2].durability
                                    + l * ingredients[3].durability,
                            ) * max(
                                0,
                                i * ingredients[0].flavor
                                    + j * ingredients[1].flavor
                                    + k * ingredients[2].flavor
                                    + l * ingredients[3].flavor,
                            ) * max(
                                0,
                                i * ingredients[0].texture
                                    + j * ingredients[1].texture
                                    + k * ingredients[2].texture
                                    + l * ingredients[3].texture,
                            ),
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
