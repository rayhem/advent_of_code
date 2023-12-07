use std::{collections::HashSet, str::FromStr};

use crate::utils::solution::Solution;

/// Puzzle: <https://adventofcode.com/2023/day/4>

pub struct Day04 {}

impl Solution for Day04 {
    /// Computes the total number of [points](Card::points) for a collection of scratchcards
    fn part_one(&self, input: &str) -> Option<String> {
        Scratchcards::from_str(input)
            .map(|sc| sc.total_points().to_string())
            .ok()
    }

    /// Computes the total number of scratchcards produced from an initial set
    fn part_two(&self, input: &str) -> Option<String> {
        Scratchcards::from_str(input)
            .map(|sc| sc.total_cards().to_string())
            .ok()
    }
}

struct Scratchcards(Vec<Card>);

impl Scratchcards {
    fn total_points(&self) -> i32 {
        self.0.iter().map(|card| card.points()).sum::<i32>()
    }

    fn total_cards(&self) -> i32 {
        let mut copies = vec![1; self.0.len()];

        for i in 0..self.0.len() {
            let n = self.0[i].num_winning_numbers() as usize;

            for j in (i + 1)..=(i + n) {
                copies[j] += copies[i];
            }
        }

        copies.iter().sum::<i32>()
    }
}

impl FromStr for Scratchcards {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter_map(|line| {
                    line.split_once(':')
                        .map(|(_, c)| Card::from_str(c).unwrap())
                })
                .collect(),
        ))
    }
}

type Numbers = HashSet<i32>;
struct Card {
    winning_numbers: Numbers,
    have_numbers: Numbers,
}

/// Representation of a single scratchcard
///

impl Card {
    fn new(winning_numbers: Numbers, have_numbers: Numbers) -> Self {
        Self {
            winning_numbers,
            have_numbers,
        }
    }

    fn num_winning_numbers(&self) -> i32 {
        self.winning_numbers
            .intersection(&self.have_numbers)
            .count() as i32
    }

    /// Compute the point value of the card. Starting at 1 point for the first
    /// matching number, the point value doubles for each additional match. A
    /// [`Card`] with no matching numbers is worth zero points.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use advent::solutions::year_2023::day_04::Card;
    ///
    /// let winning_numbers = HashSet::from([41, 48, 83, 86, 17]);
    /// let have_numbers = HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]);
    /// let card = Card::new(winning_numbers, have_numbers);
    ///
    /// // 17, 48, 83, and 86 all match, so the card is worth 8 points
    /// assert_eq!(card.points(), 8);
    /// ```
    fn points(&self) -> i32 {
        let n = self.num_winning_numbers();
        match n {
            0 => 0,
            _ => 2_i32.pow((n - 1) as u32),
        }
    }
}

impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((winning, have)) = s.split_once('|') {
            Ok(Card::new(
                winning
                    .split_whitespace()
                    .flat_map(str::parse::<i32>)
                    .collect(),
                have.split_whitespace()
                    .flat_map(str::parse::<i32>)
                    .collect(),
            ))
        } else {
            Err("bad card string")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
        let scratchcards = Scratchcards::from_str(INPUT).expect("failed to parse input");

        assert_eq!(scratchcards.0[0].points(), 8);
        assert_eq!(scratchcards.0[1].points(), 2);
        assert_eq!(scratchcards.0[2].points(), 2);
        assert_eq!(scratchcards.0[3].points(), 1);
        assert_eq!(scratchcards.0[4].points(), 0);
        assert_eq!(scratchcards.0[5].points(), 0);

        assert_eq!(scratchcards.total_points(), 13);
        assert_eq!(scratchcards.total_cards(), 30);
    }
}

crate::verify!(Day04, crate::my_input!("2023", "Day04"), "15268", "6283755");
