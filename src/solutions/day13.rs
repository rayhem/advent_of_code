use advent_utils::solution::Solution;
use std::{cmp::Ordering, collections::HashSet, error::Error, fmt::Display, str::FromStr};

pub struct Day13 {}

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            Instructions::from_str(input)
                .unwrap()
                .fold()
                .num_points()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let i = Instructions::from_str(input).unwrap().fold_all();
        println!("{}", i);

        None
    }
}

#[derive(Clone, Debug)]
struct Instructions {
    paper: HashSet<(i32, i32)>,
    folds: Vec<Fold>,
}

impl Instructions {
    fn fold(&mut self) -> &Self {
        let fold_value = |value: i32, fold: i32| match value.cmp(&fold) {
            Ordering::Greater => fold - (value - fold),
            Ordering::Less => value,
            Ordering::Equal => unreachable!(),
        };

        if let Some(fold) = self.folds.pop() {
            self.paper = self
                .paper
                .iter()
                .map(|&(x, y)| match fold {
                    Fold::Horizontal(n) => (fold_value(x, n), y),
                    Fold::Vertical(n) => (x, fold_value(y, n)),
                })
                .collect();
        }

        self
    }

    fn fold_all(mut self) -> Self {
        while !self.folds.is_empty() {
            self.fold();
        }

        self
    }

    fn num_points(&self) -> usize {
        self.paper.len()
    }
}

impl Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_x = *self.paper.iter().map(|(x, _)| x).max().unwrap();
        let max_y = *self.paper.iter().map(|(_, y)| y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.paper.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromStr for Instructions {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, folds_str) = s.split_once("\n\n").ok_or("Invalid input")?;

        let mut paper = HashSet::new();
        for line in points.lines() {
            let (x, y) = line
                .split_once(',')
                .ok_or(format!("Malformed point: {}", line))?;

            paper.insert((x.parse()?, y.parse()?));
        }

        let mut folds: Vec<Fold> = Vec::new();
        for line in folds_str.lines().rev() {
            let token = line
                .split_whitespace()
                .nth(2)
                .ok_or(format!("Malformed fold: {}", line))?;

            let (dir, num) = token.split_once('=').unwrap();

            folds.push(match dir {
                "x" => Fold::Horizontal(num.parse()?),
                "y" => Fold::Vertical(num.parse()?),
                dir => return Err(format!("Invalid fold direction: {}", dir).into()),
            });
        }

        Ok(Self { paper, folds })
    }
}

#[derive(Clone, Copy, Debug)]
enum Fold {
    Horizontal(i32),
    Vertical(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        #[test]
        fn example1() {
            let mut instructions = Instructions::from_str(DATA).unwrap();
            assert_eq!(instructions.fold().num_points(), 17);
        }

        #[test]
        fn example2() {}
    }

    mod integration {
        use super::*;
        const SOLUTION: Day13 = Day13 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day13.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("")));
        }
    }
}
