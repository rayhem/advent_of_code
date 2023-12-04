use crate::utils::solution::Solution;
use std::{collections::HashMap, str::FromStr};

pub struct Day03 {}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> Option<String> {
        let parts = Engine::from_str(input).unwrap().parts();
        Some(parts.part_total().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let parts = Engine::from_str(input).unwrap().parts();
        Some(parts.gear_total().to_string())
    }
}

#[derive(Clone, Debug)]
struct Part {
    symbol: char,
    numbers: Vec<i32>,
}

impl Part {
    fn new(ch: char) -> Self {
        Self {
            symbol: ch,
            numbers: Vec::new(),
        }
    }
}

struct EngineParts(HashMap<(usize, usize), Part>);

impl EngineParts {
    fn part_total(&self) -> i32 {
        self.0
            .values()
            .map(|part| part.numbers.iter().sum::<i32>())
            .sum::<i32>()
    }

    fn gear_total(&self) -> i32 {
        self.0
            .values()
            .filter(|part| part.symbol == '*' && part.numbers.len() == 2)
            .map(|part| part.numbers[0] * part.numbers[1])
            .sum::<i32>()
    }
}

#[derive(Clone, Debug)]
struct Engine(Vec<Vec<char>>);

impl Engine {
    fn parts(&self) -> EngineParts {
        let num_cols = match self.0.first() {
            Some(row) => row.len(),
            None => 0,
        };

        let mut parts: HashMap<(usize, usize), Part> = HashMap::new();

        for row_idx in 0..self.0.len() {
            let row = self.0.get(row_idx).unwrap();

            let mut col_idx = 0;

            'columns: while (0..num_cols).contains(&col_idx) {
                if !row[col_idx].is_numeric() {
                    col_idx += 1;
                    continue;
                }

                let mut word_len = 0;
                while col_idx + word_len < row.len() && row[col_idx + word_len].is_numeric() {
                    word_len += 1;
                }

                let lower_row = ((row_idx as i32) - 1).max(0) as usize;
                let upper_row = (row_idx + 1).min(self.0.len() - 1);

                let lower_col = ((col_idx as i32) - 1).max(0) as usize;
                let upper_col = (col_idx + word_len).min(num_cols - 1);

                for adjacent_row in lower_row..=upper_row {
                    for adjacent_col in lower_col..=upper_col {
                        if adjacent_row == row_idx
                            && (col_idx..(col_idx + word_len)).contains(&adjacent_col)
                        {
                            continue;
                        }

                        let adjacent_char = self.0[adjacent_row][adjacent_col];
                        let is_symbol = !adjacent_char.is_numeric() && adjacent_char != '.';

                        if !is_symbol {
                            continue;
                        }

                        let number = self.0[row_idx][col_idx..(col_idx + word_len)]
                            .iter()
                            .collect::<String>();

                        parts
                            .entry((adjacent_row, adjacent_col))
                            .or_insert(Part::new(adjacent_char))
                            .numbers
                            .push(number.parse::<i32>().expect("expected numeric string"));

                        col_idx += word_len;
                        continue 'columns;
                    }
                }

                col_idx += word_len + 1;
            }
        }

        EngineParts(parts)
    }
}

impl FromStr for Engine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(|line| line.chars().collect()).collect()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let parts = Engine::from_str(INPUT).unwrap().parts();
        assert_eq!(parts.part_total(), 4361);
        assert_eq!(parts.gear_total(), 467835);
    }
}

crate::verify!(
    Day03,
    crate::my_input!("2023", "Day03"),
    "535351",
    "87287096"
);
