use advent_utils::solution::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day05 {}

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(vent_counts(input, |s| !s.is_diagonal()).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(vent_counts(input, |_| true).to_string())
    }
}

fn vent_counts(s: &str, filter_fn: fn(&Segment) -> bool) -> usize {
    s.lines()
        .flat_map(|line| line.parse::<Segment>())
        .filter(filter_fn)
        .flat_map(|segment| segment.vents())
        .counts()
        .into_iter()
        .filter(|(_, num_occurrences)| *num_occurrences > 1)
        .count()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s
            .split_once(',')
            .ok_or(format!("Malformed coordinate: {}", s))?;

        Ok(Point {
            x: x_str.parse()?,
            y: y_str.parse()?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    start: Point,
    end: Point,
}

impl Segment {
    fn is_diagonal(&self) -> bool {
        !(self.start.x == self.end.x || self.start.y == self.end.y)
    }

    fn vents(&self) -> impl Iterator<Item = Point> {
        let num_x = (self.end.x - self.start.x).abs();
        let num_y = (self.end.y - self.start.y).abs();
        let num_vents = num_x.max(num_y) + 1;
        let dx = (self.end.x - self.start.x).signum();
        let dy = (self.end.y - self.start.y).signum();

        let start_x = self.start.x;
        let start_y = self.start.y;

        (0..num_vents).map(move |i| Point {
            x: start_x + i * dx,
            y: start_y + i * dy,
        })
    }
}

impl FromStr for Segment {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s
            .split_once(" -> ")
            .ok_or(format!("Malformed segment string: {}", s))?;

        Ok(Self {
            start: start_str.parse()?,
            end: end_str.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

        #[test]
        fn example1() {
            assert_eq!(vent_counts(DATA, |s| !s.is_diagonal()), 5)
        }

        #[test]
        fn example2() {
            assert_eq!(vent_counts(DATA, |_| true), 12)
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day05 = Day05 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day05.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("5124")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("19771")));
        }
    }
}
