use advent_utils::solution::Solution;
use std::str::FromStr;

pub struct Day10 {}

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(corrupted_score(input).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(autocomplete_score(input).to_string())
    }
}

fn corrupted_score(s: &str) -> i64 {
    s.lines()
        .flat_map(ChunkStatus::from_str)
        .map(|c| match c {
            ChunkStatus::Corrupted(i) => i,
            _ => 0,
        })
        .sum()
}

fn autocomplete_score(s: &str) -> i64 {
    let mut scores = s
        .lines()
        .flat_map(ChunkStatus::from_str)
        .filter_map(|c| match c {
            ChunkStatus::Incomplete(i) => Some(i),
            _ => None,
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Grouping {
    Paren,
    Bracket,
    Brace,
    Angle,
}

impl Grouping {
    fn error_score(&self) -> i64 {
        match self {
            Grouping::Paren => 3,
            Grouping::Bracket => 57,
            Grouping::Brace => 1197,
            Grouping::Angle => 25137,
        }
    }

    fn autocomplete_score(&self) -> i64 {
        match self {
            Grouping::Paren => 1,
            Grouping::Bracket => 2,
            Grouping::Brace => 3,
            Grouping::Angle => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Symbol {
    Opening(Grouping),
    Closing(Grouping),
}

impl Symbol {
    fn unwrap(&self) -> Grouping {
        match *self {
            Self::Opening(g) => g,
            Self::Closing(g) => g,
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match ['(', '[', '{', '<'].contains(&value) {
            true => Ok(Symbol::Opening(match value {
                '(' => Grouping::Paren,
                '[' => Grouping::Bracket,
                '{' => Grouping::Brace,
                '<' => Grouping::Angle,
                _ => unreachable!(),
            })),
            false => Ok(Symbol::Closing(match value {
                ')' => Grouping::Paren,
                ']' => Grouping::Bracket,
                '}' => Grouping::Brace,
                '>' => Grouping::Angle,
                _ => return Err(format!("Invalid token: {}", value).into()),
            })),
        }
    }
}

impl Symbol {
    fn matches(&self, other: &Self) -> bool {
        if let Symbol::Opening(a) = self {
            if let Symbol::Closing(b) = other {
                return a == b;
            }
        }

        false
    }
}

#[derive(Clone, Copy, Debug)]
enum ChunkStatus {
    Corrupted(i64),
    Incomplete(i64),
}

impl FromStr for ChunkStatus {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let symbols = s.chars().map(|ch| Symbol::try_from(ch).unwrap());
        let mut stack: Vec<Symbol> = Vec::default();

        for symbol in symbols {
            if let Some(last) = stack.last() {
                if last.matches(&symbol) {
                    stack.pop();
                } else {
                    match symbol {
                        Symbol::Opening(_) => stack.push(symbol),
                        Symbol::Closing(g) => return Ok(Self::Corrupted(g.error_score())),
                    }
                }
            } else {
                stack.push(symbol);
            }
        }

        Ok(Self::Incomplete(stack.iter().rev().fold(0, |sum, s| {
            5 * sum + s.unwrap().autocomplete_score()
        })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

        #[test]
        fn example1() {
            assert_eq!(corrupted_score(DATA), 26397);
        }

        #[test]
        fn example2() {
            assert_eq!(autocomplete_score(DATA), 288957);
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day10 = Day10 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day10.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("390993")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("94017638")));
        }
    }
}
