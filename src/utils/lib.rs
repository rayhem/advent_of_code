pub mod cli;
pub mod solution;

#[cfg(feature = "verification")]
#[macro_export]
macro_rules! my_input {
    ($year:expr, $day:expr) => {
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/inputs/",
            $year,
            "/day",
            $day,
            ".txt"
        )
    };
}

#[cfg(feature = "verification")]
#[macro_export]
macro_rules! verify {
    ($x:ident, $input_file:expr, $sol1:expr, $sol2:expr) => {
        #[cfg(test)]
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
