use crate::utils::solution::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(sum_part_adjacent_numbers(&char_grid(input)).to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn char_grid(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| line.chars().collect()).collect()
}

fn sum_part_adjacent_numbers(char_grid: &Vec<Vec<char>>) -> i32 {
    let num_cols = match char_grid.first() {
        Some(row) => row.len(),
        None => 0,
    };

    let mut total = 0;

    for row_idx in 0..char_grid.len() {
        let row = char_grid.get(row_idx).unwrap();

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
            let upper_row = (row_idx + 1).min(char_grid.len() - 1);

            let lower_col = ((col_idx as i32) - 1).max(0) as usize;
            let upper_col = (col_idx + word_len).min(num_cols - 1);

            for adjacent_row in lower_row..=upper_row {
                for adjacent_col in lower_col..=upper_col {
                    if adjacent_row == row_idx
                        && (col_idx..(col_idx + word_len)).contains(&adjacent_col)
                    {
                        continue;
                    }

                    let adjacent_char = char_grid[adjacent_row][adjacent_col];
                    let is_symbol = !adjacent_char.is_numeric() && adjacent_char != '.';

                    if !is_symbol {
                        continue;
                    }

                    let number = char_grid[row_idx][col_idx..(col_idx + word_len)]
                        .iter()
                        .collect::<String>();

                    println!("{number}");

                    total += number.parse::<i32>().expect("expected numeric string");
                    col_idx += word_len;
                    continue 'columns;
                }
            }

            col_idx += word_len + 1;
        }
    }

    total
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
        assert_eq!(sum_part_adjacent_numbers(&char_grid(INPUT)), 4361);
    }
}

crate::verify!(Day03, crate::my_input!("2023", "Day03"), "", "");
