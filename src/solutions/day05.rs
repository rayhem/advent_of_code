use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day05 {}

#[derive(Clone, Copy, Debug)]
enum Day05Error {
    WalkNotConverged,
}

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(input.lines().map(|l| seat_id(l)).max().unwrap().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let passes: HashSet<_> = input.lines().map(|l| seat_id(l)).collect();
        let seats: HashSet<_> = (0..(127 * 8)).collect();

        let mut unfilled: Vec<_> = seats.difference(&passes).into_iter().collect();
        unfilled.sort();

        unfilled
            .iter()
            .tuple_windows()
            .find(|(a, b, c)| ***b != ***a + 1 && ***c != ***b + 1)
            .map(|(_, seat, _)| seat.to_string())
    }
}

fn seat_id(s: &str) -> i32 {
    let row_steps: Vec<_> = s.chars().take(7).collect();
    let col_steps: Vec<_> = s.chars().skip(7).collect();

    walk(
        &row_steps,
        Bounds {
            lower: 0,
            upper: 127,
        },
    )
    .unwrap()
        * 8
        + walk(&col_steps, Bounds { lower: 0, upper: 7 }).unwrap()
}

#[derive(Clone, Copy, Debug)]
struct Bounds {
    lower: i32,
    upper: i32,
}

impl Bounds {
    fn mid(&self) -> i32 {
        (self.lower + self.upper) / 2
    }
}

fn walk(s: &Vec<char>, mut bounds: Bounds) -> Result<i32, Day05Error> {
    for c in s {
        match c {
            'F' | 'L' => bounds.upper = bounds.mid(),
            'B' | 'R' => bounds.lower = bounds.mid() + 1,
            _ => (),
        }
    }

    if bounds.lower == bounds.upper {
        Ok(bounds.lower)
    } else {
        Err(Day05Error::WalkNotConverged)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_passes() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}
