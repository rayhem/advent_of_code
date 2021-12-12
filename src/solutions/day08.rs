use advent_utils::solution::Solution;

pub struct Day08 {}

impl Solution for Day08 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(count_unique_segments(input).to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn count_unique_segments(s: &str) -> i32 {
    s.lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(_, output)| {
            output
                .split_whitespace()
                .map(|s| s.trim().len())
                .filter(|&l| l == 2 || l == 4 || l == 3 || l == 7)
                .count() as i32
        })
        .sum()
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
