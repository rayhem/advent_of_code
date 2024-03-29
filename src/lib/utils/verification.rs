#[macro_export]
macro_rules! my_input {
    ($year:expr, $day:expr) => {
        concat!(env!("ADVENT_INPUTS"), '/', $year, "/day", $day, ".txt")
    };
}

#[macro_export]
macro_rules! verify {
    ($x:ident, $input_file:expr, $sol1:expr, $sol2:expr) => {
        #[cfg(test)]
        #[cfg(feature = "verification")]
        mod verification {
            use super::*;

            const SOLUTION: $x = $x {};
            static INPUT: &str = include_str!($input_file);

            #[test]
            fn part_01() {
                assert_eq!(SOLUTION.part_one(INPUT), Some($sol1.to_owned()));
            }

            #[test]
            fn part_02() {
                assert_eq!(SOLUTION.part_two(INPUT), Some($sol2.to_owned()));
            }
        }
    };
}
