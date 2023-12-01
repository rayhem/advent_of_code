use std::ops::Index;

use crate::utils::solution::Solution;

pub struct Day01 {}

const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

const WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .filter_map(calibration_value_from_digits)
                .sum::<usize>()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .filter_map(calibration_value_from_words)
                .sum::<usize>()
                .to_string(),
        )
    }
}

fn calibration_value_from_digits(s: &str) -> Option<usize> {
    let first = s.find(char::is_numeric)?;
    let last = s.rfind(char::is_numeric)?;

    let tens = s.as_bytes()[first] - b'0';
    let ones = s.as_bytes()[last] - b'0';

    Some(10 * (tens as usize) + (ones as usize))
}

fn calibration_value_from_words(s: &str) -> Option<usize> {
    let (first, _) = WORDS
        .iter()
        .enumerate()
        .chain(DIGITS.iter().enumerate())
        .min_by_key(|(_, num)| s.find(**num).unwrap_or(usize::MAX))?;

    let (last, _) = WORDS
        .iter()
        .enumerate()
        .chain(DIGITS.iter().enumerate())
        .max_by_key(|(_, num)| s.rfind(**num).map(|i| i as i32).unwrap_or(i32::MIN))?;

    Some(10 * first + last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_digits() {
        assert_eq!(calibration_value_from_digits("1abc2"), Some(12));
        assert_eq!(calibration_value_from_digits("pqr3stu8vwx"), Some(38));
        assert_eq!(calibration_value_from_digits("a1b2c3d4e5f"), Some(15));
        assert_eq!(calibration_value_from_digits("treb7uchet"), Some(77));
    }

    #[test]
    fn example_text() {
        assert_eq!(calibration_value_from_words("two1nine"), Some(29));
        assert_eq!(calibration_value_from_words("eightwothree"), Some(83));
        assert_eq!(calibration_value_from_words("abcone2threexyz"), Some(13));
        assert_eq!(calibration_value_from_words("xtwone3four"), Some(24));
        assert_eq!(calibration_value_from_words("4nineeightseven2"), Some(42));
        assert_eq!(calibration_value_from_words("zoneight234"), Some(14));
        assert_eq!(calibration_value_from_words("7pqrstsixteen"), Some(76));
    }
}

crate::verify!(Day01, crate::my_input!("2023", "01"), "55712", "55413");
