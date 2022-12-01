use utils::solution::Solution;
use regex::Regex;

pub struct Day05 {}

impl Solution for Day05 {
    fn part_one(&self, input: &str) -> Option<String> {
        let passwords = input.split_whitespace();
        Some(passwords.filter(|s| is_nice(*s)).count().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let passwords = input.split_whitespace();
        Some(passwords.filter(|s| is_nice_v2(*s)).count().to_string())
    }
}

fn has_three_vowels(password: &str) -> bool {
    let re = Regex::new(".*[aeiou].*[aeiou].*[aeiou].*").unwrap();
    re.is_match(password)
}

fn has_duplicate(password: &str) -> bool {
    let re = Regex::new(".{2,}").unwrap();
    re.is_match(password)
}

fn no_bad_strings(password: &str) -> bool {
    let re = Regex::new("").unwrap();
    re.is_match(password)
}

fn is_nice(password: &str) -> bool {
    has_three_vowels(password) && has_duplicate(password) && no_bad_strings(password)
}

fn is_nice_v2(_password: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {
    mod part1 {
        use super::super::*;

        #[test]
        fn example0() {
            assert!(is_nice("ugknbfddgicrmopn"));
        }

        #[test]
        fn example1() {
            assert!(is_nice("aaa"));
        }

        #[test]
        fn example2() {
            assert!(!is_nice("jchzalrnumimnmhp"));
        }

        #[test]
        fn example3() {
            assert!(!is_nice("haegwjzuvuyypxyu"));
        }

        #[test]
        fn example4() {
            assert!(!is_nice("dvszwmarrgswjxmb"));
        }
    }
    mod part2 {
        use super::super::*;

        #[test]
        fn example0() {
            assert!(is_nice_v2("qjhvhtzxzqqjkmpb"));
        }

        #[test]
        fn example1() {
            assert!(is_nice_v2("xxyxx"));
        }

        #[test]
        fn example2() {
            assert!(!is_nice_v2("uurcxstgmygtbstg"));
        }

        #[test]
        fn example3() {
            assert!(!is_nice_v2("ieodomkazucvgmuy"));
        }
    }
}
