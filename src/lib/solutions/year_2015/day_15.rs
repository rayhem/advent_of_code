use crate::utils::solution::Solution;
use std::str::{FromStr, SplitWhitespace};

pub struct Day15 {}

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> Option<String> {
        let ingredients = ingredients(input);

        let mut result: i32 = 0;

        for i in 0..=100 {
            for j in 0..=(100 - i) {
                for k in 0..=(100 - (i + j)) {
                    let l = 100 - (i + j + k);

                    result = result.max(score(&ingredients, &vec![i, j, k, l]));
                }
            }
        }

        Some(result.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let ingredients = ingredients(input);

        let mut result: i32 = 0;

        for i in 0..=100 {
            for j in 0..=(100 - i) {
                for k in 0..=(100 - (i + j)) {
                    let l = 100 - (i + j + k);

                    let total_calories = [i, j, k, l]
                        .iter()
                        .enumerate()
                        .map(|(idx, value)| ingredients[idx].calories * value)
                        .sum::<i32>();

                    if total_calories != 500 {
                        continue;
                    }

                    result = result.max(score(&ingredients, &vec![i, j, k, l]));
                }
            }
        }

        Some(result.to_string())
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl FromStr for Ingredient {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();

        let next_val = |token: &mut SplitWhitespace<'_>| {
            token
                .nth(1)
                .unwrap()
                .trim_end_matches(',')
                .parse::<i32>()
                .unwrap()
        };

        Ok(Ingredient {
            capacity: next_val(&mut tokens),
            durability: next_val(&mut tokens),
            flavor: next_val(&mut tokens),
            texture: next_val(&mut tokens),
            calories: next_val(&mut tokens),
        })
    }
}

fn ingredients(s: &str) -> Vec<Ingredient> {
    s.lines()
        .filter_map(|line| {
            line.split_once(':')
                .map(|(_, a)| Ingredient::from_str(a).ok())
                .unwrap()
        })
        .collect()
}

fn score(ingredients: &[Ingredient], quantities: &Vec<i32>) -> i32 {
    let mut totals = [0; 5];
    for (ingredient, &quantity) in ingredients.iter().zip(quantities) {
        totals[0] += ingredient.capacity * quantity;
        totals[1] += ingredient.durability * quantity;
        totals[2] += ingredient.flavor * quantity;
        totals[3] += ingredient.texture * quantity;
        totals[4] += ingredient.calories * quantity;
    }

    totals
        .iter()
        .take(4)
        .map(|&value| value.max(0))
        .product::<i32>()
}

crate::verify!(Day15, crate::my_input!("2015", "15"), "", "");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        const INPUT: &str =
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let ingredients = ingredients(INPUT);

        assert_eq!(score(&ingredients, &vec![44, 56]), 62842880);
    }
}
