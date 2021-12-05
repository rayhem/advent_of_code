use advent_utils::solution::Solution;
use std::{fmt::Display, str::FromStr};

pub struct Day04 {}

impl Solution for Day04 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (numbers, mut boards) = parse_input(input);

        for number in numbers.into_iter() {
            boards.iter_mut().for_each(|board| board.mark(number));
            if let Some(board) = boards.iter().find(|board| board.is_winner()) {
                return Some((board.unmarked_sum() * number).to_string());
            }
        }

        unreachable!()
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let (numbers, mut boards) = parse_input(input);
        let mut winning_boards: Vec<(i32, Board)> = Vec::new();
        winning_boards.reserve(boards.len());

        for number in numbers.into_iter() {
            boards.iter_mut().for_each(|b| b.mark(number));

            let mut i = 0;
            while i < boards.len() {
                if boards[i].is_winner() {
                    winning_boards.push((number, boards.remove(i)));
                } else {
                    i += 1;
                }
            }
        }

        let (number, board) = winning_boards.pop().unwrap();
        Some((number * board.unmarked_sum()).to_string())
    }
}

fn parse_input(s: &str) -> (Vec<i32>, Vec<Board>) {
    let (numbers_str, boards_str) = s.split_once('\n').unwrap();

    (
        numbers_str
            .split(',')
            .flat_map(|token| token.parse())
            .collect(),
        boards_str
            .split("\n\n")
            .flat_map(|s| Board::from_str(s.trim()))
            .collect(),
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BingoState {
    Marked(i32),
    Unmarked(i32),
}

impl Default for BingoState {
    fn default() -> Self {
        Self::Unmarked(0)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Board {
    data: [[BingoState; 5]; 5],
}

impl Board {
    fn is_winner(&self) -> bool {
        let winning_row = self
            .data
            .iter()
            .any(|row| row.iter().all(|&col| matches!(col, BingoState::Marked(_))));

        let winning_col = (0..5).any(|col_idx| {
            self.data
                .iter()
                .all(|row| matches!(row[col_idx], BingoState::Marked(_)))
        });

        winning_row || winning_col
    }

    fn mark(&mut self, value: i32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.data[row][col] == BingoState::Unmarked(value) {
                    self.data[row][col] = BingoState::Marked(value);
                }
            }
        }
    }

    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                sum += match self.data[i][j] {
                    BingoState::Unmarked(n) => n,
                    BingoState::Marked(_) => 0,
                };
            }
        }

        sum
    }
}

impl FromStr for Board {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::default();

        for (i, line) in s.lines().enumerate() {
            for (j, value) in line.split_whitespace().enumerate() {
                board.data[i][j] = BingoState::Unmarked(value.parse()?);
            }
        }

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                match self.data[row][col] {
                    BingoState::Marked(n) => write!(f, "{:x>5} ", n)?,
                    BingoState::Unmarked(n) => write!(f, "{:>5} ", n)?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        #[test]
        fn example1() {
            let solution = Day04 {};
            assert_eq!(solution.part_one(DATA), Some(String::from("4512")));
        }

        #[test]
        fn example2() {
            let solution = Day04 {};
            assert_eq!(solution.part_two(DATA), Some(String::from("1924")));
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day04 = Day04 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day04.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("21607")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("19012")));
        }
    }
}
