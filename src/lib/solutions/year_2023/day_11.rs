use itertools::Itertools;

use crate::utils::solution::Solution;
use std::{collections::HashSet, str::FromStr};

pub struct Day11 {}

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> Option<String> {
        solve(input, 2)
    }

    fn part_two(&self, input: &str) -> Option<String> {
        solve(input, 1000000)
    }
}

fn solve(input: &str, expansion_factor: u64) -> Option<String> {
    Image::from_str(input)
        .map(|image| image.pairwise_total(expansion_factor).to_string())
        .ok()
}

type Position = glam::I64Vec2;

#[derive(Clone, Debug)]
struct Image {
    galaxies: Vec<Position>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
}

impl Image {
    fn pairwise_distance(&self, pos1: &Position, pos2: &Position, expansion_factor: u64) -> u64 {
        let rows = (pos1.x.min(pos2.x) as usize)..=(pos1.x.max(pos2.x) as usize);
        let cols = (pos1.y.min(pos2.y) as usize)..=(pos1.y.max(pos2.y) as usize);

        let num_empty_rows_between = self.empty_rows.intersection(&rows.collect()).count() as u64;
        let num_empty_cols_between = self.empty_cols.intersection(&cols.collect()).count() as u64;

        pos1.x.abs_diff(pos2.x)
            + pos1.y.abs_diff(pos2.y)
            + (expansion_factor - 1) * (num_empty_rows_between + num_empty_cols_between)
    }

    fn pairwise_total(&self, expansion_factor: u64) -> u64 {
        self.galaxies
            .iter()
            .tuple_combinations()
            .map(|(pos1, pos2)| self.pairwise_distance(pos1, pos2, expansion_factor))
            .sum()
    }
}

impl FromStr for Image {
    type Err = crate::utils::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_rows = s.lines().count();
        let num_cols = s.lines().next().unwrap().chars().count();

        let mut galaxies = Vec::new();
        let mut empty_rows = (0..num_rows).collect::<HashSet<_>>();
        let mut empty_cols = (0..num_cols).collect::<HashSet<_>>();

        let chars = s
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .collect::<Vec<_>>();

        for row in 0..num_rows {
            for col in 0..num_cols {
                let idx = row * num_cols + col;
                match chars.get(idx) {
                    Some('#') => {
                        galaxies.push(glam::I64Vec2::new(row as i64, col as i64));
                        empty_rows.remove(&row);
                        empty_cols.remove(&col);
                    }
                    None | Some(_) => continue,
                }
            }
        }

        Ok(Self {
            galaxies,
            empty_rows,
            empty_cols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

        let image = Image::from_str(INPUT).unwrap();
        assert_eq!(
            image.pairwise_distance(&image.galaxies[4], &image.galaxies[8], 2),
            9
        );
        assert_eq!(
            image.pairwise_distance(&image.galaxies[0], &image.galaxies[6], 2),
            15
        );
        assert_eq!(
            image.pairwise_distance(&image.galaxies[2], &image.galaxies[5], 2),
            17
        );
        assert_eq!(
            image.pairwise_distance(&image.galaxies[7], &image.galaxies[8], 2),
            5
        );

        assert_eq!(image.pairwise_total(2), 374);

        assert_eq!(image.pairwise_total(10), 1030);
        assert_eq!(image.pairwise_total(100), 8410);
    }
}

crate::verify!(
    Day11,
    crate::my_input!("2023", "Day11"),
    "9918828",
    "692506533832"
);
