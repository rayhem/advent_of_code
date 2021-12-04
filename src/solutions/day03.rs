use advent_utils::solution::Solution;

pub struct Day03 {}

impl Solution for Day03 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (gamma_rate, epsilon_rate) = rates(&parse_input(input));
        Some((gamma_rate * epsilon_rate).to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn parse_input(s: &str) -> Vec<Vec<i32>> {
    s.lines()
        .map(|l| l.chars().map(|c| c.to_digit(2).unwrap() as i32).collect())
        .collect()
}

fn rates(numbers: &[Vec<i32>]) -> (i32, i32) {
    let frequencies = numbers
        .iter()
        .fold(vec![0; numbers[0].len()], |mut frequencies, digits| {
            frequencies
                .iter_mut()
                .zip(digits.iter())
                .for_each(|(frequency, digit)| *frequency += *digit);
            frequencies
        });

    let most_common_digit = frequencies.into_iter().map(|frequency| {
        match (frequency as usize).cmp(&(numbers.len() / 2)) {
            std::cmp::Ordering::Equal => unreachable!(),
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Greater => 1,
        }
    });

    let gamma_rate = most_common_digit
        .clone()
        .rev()
        .enumerate()
        .fold(0, |a, (i, b)| a + b * 2_i32.pow(i.try_into().unwrap()));

    let epsilon_rate = most_common_digit.rev().enumerate().fold(0, |a, (i, b)| {
        a + (1 - b) * 2_i32.pow(i.try_into().unwrap())
    });

    (gamma_rate, epsilon_rate)
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
            let numbers = parse_input(DATA);
            let (gamma_rate, epsilon_rate) = rates(&numbers);
            assert_eq!(gamma_rate, 22);
            assert_eq!(epsilon_rate, 9);
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day03 = Day03 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day03.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("1540244")));
        }
    }
}
