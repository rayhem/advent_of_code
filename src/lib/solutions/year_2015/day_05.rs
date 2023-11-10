use crate::utils::solution::Solution;
use itertools::Itertools;

pub struct Day05 {}

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> Option<String> {
        let passwords = input.split_whitespace();
        Some(passwords.filter(|s| is_nice(s)).count().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let passwords = input.split_whitespace();
        Some(passwords.filter(|s| is_nice_v2(s)).count().to_string())
    }
}

fn has_three_vowels(password: &str) -> bool {
    let vowel_count = password.chars().filter(|ch| "aeiou".contains(*ch)).count();
    vowel_count >= 3
}

fn has_duplicate(password: &str) -> bool {
    password.chars().tuple_windows().any(|(a, b)| a == b)
}

fn has_bad_strings(password: &str) -> bool {
    ["ab", "cd", "pq", "xy"]
        .iter()
        .any(|invalid| password.contains(invalid))
}

fn is_nice(password: &str) -> bool {
    has_three_vowels(password) && has_duplicate(password) && !has_bad_strings(password)
}

fn pair_occurs_twice(password: &str) -> bool {
    for i in 0..(password.len() - 1) {
        let s = &password[i..i + 2];
        if password[(i + 2)..].contains(s) {
            return true;
        }
    }

    false
}

fn has_sandwich(password: &str) -> bool {
    password.chars().tuple_windows().any(|(a, _, b)| a == b)
}

fn is_nice_v2(password: &str) -> bool {
    pair_occurs_twice(password) && has_sandwich(password)
}

crate::verify!(Day05, crate::my_input!("2015", "05"), "258", "53");

#[cfg(test)]
mod tests {
    mod part1 {
        use super::super::*;

        #[test]
        fn example0() {
            const INPUT: &str = "ugknbfddgicrmopn";
            assert!(has_three_vowels(INPUT));
            assert!(has_duplicate(INPUT));
            assert!(!has_bad_strings(INPUT));
            assert!(is_nice(INPUT));
        }

        #[test]
        fn example1() {
            const INPUT: &str = "aaa";
            assert!(has_three_vowels(INPUT));
            assert!(has_duplicate(INPUT));
            assert!(!has_bad_strings(INPUT));
            assert!(is_nice(INPUT));
        }

        #[test]
        fn example2() {
            const INPUT: &str = "jchzalrnumimnmhp";
            assert!(has_three_vowels(INPUT));
            assert!(!has_duplicate(INPUT));
            assert!(!has_bad_strings(INPUT));
            assert!(!is_nice(INPUT));
        }

        #[test]
        fn example3() {
            const INPUT: &str = "haegwjzuvuyypxyu";
            assert!(has_three_vowels(INPUT));
            assert!(has_duplicate(INPUT));
            assert!(has_bad_strings(INPUT));
            assert!(!is_nice(INPUT));
        }

        #[test]
        fn example4() {
            const INPUT: &str = "dvszwmarrgswjxmb";
            assert!(!has_three_vowels(INPUT));
            assert!(has_duplicate(INPUT));
            assert!(!has_bad_strings(INPUT));
            assert!(!is_nice(INPUT));
        }
    }
    mod part2 {
        use super::super::*;

        #[test]
        fn example0() {
            const INPUT: &str = "qjhvhtzxzqqjkmpb";
            assert!(pair_occurs_twice(INPUT));
            assert!(has_sandwich(INPUT));
            assert!(is_nice_v2(INPUT));
        }

        #[test]
        fn example1() {
            const INPUT: &str = "xxyxx";
            assert!(pair_occurs_twice(INPUT));
            assert!(has_sandwich(INPUT));
            assert!(is_nice_v2(INPUT));
        }

        #[test]
        fn example2() {
            const INPUT: &str = "uurcxstgmygtbstg";
            assert!(pair_occurs_twice(INPUT));
            assert!(!has_sandwich(INPUT));
            assert!(!is_nice_v2(INPUT));
        }

        #[test]
        fn example3() {
            const INPUT: &str = "ieodomkazucvgmuy";
            assert!(!pair_occurs_twice(INPUT));
            assert!(has_sandwich(INPUT));
            assert!(!is_nice_v2(INPUT));
        }
    }
}
