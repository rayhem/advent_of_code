use advent_utils::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        parse_and_evaluate(input, 1)
    }

    fn part_two(&self, input: &str) -> Option<String> {
        parse_and_evaluate(input, 3)
    }
}

fn parse_and_evaluate(input: &str, skip_size: usize) -> Option<String> {
    Some(
        get_positive_deltas(input.lines().map(|l| l.parse::<i32>().unwrap()), skip_size)
            .to_string(),
    )
}

fn get_positive_deltas<T, U>(xs: T, skip_size: usize) -> usize
where
    T: Iterator<Item = U> + Clone,
    U: PartialOrd,
{
    xs.clone()
        .zip(xs.skip(skip_size))
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    mod unit {
        use super::*;
        static DATA: [i32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        #[test]
        fn adjacent_increase() {
            assert_eq!(get_positive_deltas(DATA.into_iter(), 1), 7)
        }

        #[test]
        fn three_element_increase() {
            assert_eq!(get_positive_deltas(DATA.into_iter(), 3), 5)
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day01 = Day01 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day01.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("1292")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("1262")));
        }
    }
}
