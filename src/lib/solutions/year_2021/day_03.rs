use crate::utils::solution::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> Option<String> {
        let digits = most_common_digit(&parse_input(input));
        Some((gamma_rate(&digits) * epsilon_rate(&digits)).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let numbers = parse_input(input);
        Some((oxygen_generator_rating(numbers.clone()) * co2_scrubber_rating(numbers)).to_string())
    }
}

fn parse_input(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(2).unwrap() as i32).collect())
        .collect()
}

fn most_common_digit(numbers: &[Vec<i32>]) -> Vec<i32> {
    numbers
        .iter()
        .fold(vec![0; numbers[0].len()], |mut frequencies, digits| {
            frequencies
                .iter_mut()
                .zip(digits.iter())
                .for_each(|(frequency, digit)| *frequency += *digit);
            frequencies
        })
        .iter()
        .map(
            |frequency| match (2 * frequency).cmp(&(numbers.len() as i32)) {
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Greater => 1,
            },
        )
        .collect()
}

fn gamma_rate(most_common_digits: &[i32]) -> i32 {
    most_common_digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, (i, b)| a + b * 2_i32.pow(i.try_into().unwrap()))
}

fn epsilon_rate(most_common_digits: &[i32]) -> i32 {
    most_common_digits
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, (i, b)| {
            a + (1 - b) * 2_i32.pow(i.try_into().unwrap())
        })
}

fn oxygen_generator_rating(mut numbers: Vec<Vec<i32>>) -> i32 {
    let mut i = 0;
    while numbers.len() > 1 {
        let most_common_digits = most_common_digit(&numbers);
        numbers.retain(|number| number[i] == most_common_digits[i]);
        i += 1;
    }

    numbers
        .pop()
        .unwrap()
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, (i, b)| a + b * 2_i32.pow(i.try_into().unwrap()))
}

fn co2_scrubber_rating(mut numbers: Vec<Vec<i32>>) -> i32 {
    let mut i = 0;
    while numbers.len() > 1 {
        let most_common_digits = most_common_digit(&numbers);
        numbers.retain(|number| number[i] != most_common_digits[i]);
        i += 1;
    }

    numbers
        .pop()
        .unwrap()
        .iter()
        .rev()
        .enumerate()
        .fold(0, |a, (i, b)| a + b * 2_i32.pow(i.try_into().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str =
            "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

        #[test]
        fn example1() {
            let numbers = most_common_digit(&parse_input(DATA));
            assert_eq!(gamma_rate(&numbers), 22);
            assert_eq!(epsilon_rate(&numbers), 9);
        }

        #[test]
        fn example2() {
            let numbers = parse_input(DATA);
            assert_eq!(co2_scrubber_rating(numbers.clone()), 10);
            assert_eq!(oxygen_generator_rating(numbers), 23);
        }
    }

    crate::verify!(Day03, crate::my_input!("2021", "03"), "1540244", "4203981");
}
