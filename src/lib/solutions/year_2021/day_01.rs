use crate::utils::solution::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(parse_and_evaluate(input, 1).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(parse_and_evaluate(input, 3).to_string())
    }
}

fn parse_and_evaluate(input: &str, skip_size: usize) -> usize {
    get_positive_deltas::<_, i32>(input.lines().map(|l| l.parse().unwrap()), skip_size)
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

    crate::verify!(Day01, crate::my_input!("2021", "01"), "1292", "1262");
}
