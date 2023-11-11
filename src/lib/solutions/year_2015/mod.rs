use super::DayAssociations;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;

pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;

pub mod day_23;

pub fn solutions() -> DayAssociations {
    let associations: Vec<(i32, super::BoxedSolution)> = vec![
        (1, Box::new(day_01::Day01 {})),
        (2, Box::new(day_02::Day02 {})),
        (3, Box::new(day_03::Day03 {})),
        (4, Box::new(day_04::Day04 {})),
        (5, Box::new(day_05::Day05 {})),
        (6, Box::new(day_06::Day06 {})),
        (7, Box::new(day_07::Day07 {})),
        (8, Box::new(day_08::Day08 {})),
        (9, Box::new(day_09::Day09 {})),
        (10, Box::new(day_10::Day10 {})),
        (11, Box::new(day_11::Day11 {})),
        (12, Box::new(day_12::Day12 {})),
        (13, Box::new(day_13::Day13 {})),
        (14, Box::new(day_14::Day14 {})),
        (17, Box::new(day_17::Day17 {})),
        (18, Box::new(day_18::Day18 {})),
        (19, Box::new(day_19::Day19 {})),
        (20, Box::new(day_20::Day20 {})),
        (23, Box::new(day_23::Day23 {})),
    ];

    associations.into_iter().collect()
}
