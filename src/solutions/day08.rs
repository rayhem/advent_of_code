use std::str::FromStr;

use advent_utils::solution::Solution;
use itertools::Itertools;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> Option<String> {
        None
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn count_unique_segments(s: &str) -> i32 {
    s.lines()
        .map(|line| Display::from_str(line).unwrap())
        .map(|display| {
            display
                .output_digits
                .iter()
                .filter(|i| [1, 4, 7, 8].contains(&i.len()))
                .inspect(|s| println!("{}", s))
                .count() as i32
        })
        .sum::<i32>()
}

enum Digit {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl FromStr for Digit {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

struct Display {
    input_signals: Vec<String>,
    output_digits: Vec<String>,
}

impl FromStr for Display {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (input, digits) = s
            .split_once(" | ")
            .ok_or(format!("Malformed string: {}", s))?;

        Ok(Display {
            input_signals: input.split_whitespace().map(String::from).collect(),
            output_digits: digits.split_whitespace().map(String::from).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        #[test]
        fn example1() {
            assert_eq!(count_unique_segments(DATA), 26);
        }

        #[test]
        fn example2() {}
    }

    mod integration {
        use super::*;
        const SOLUTION: Day08 = Day08 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day08.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("333755")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("94017638")));
        }
    }
}
