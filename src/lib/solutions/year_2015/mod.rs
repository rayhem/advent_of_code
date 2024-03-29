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
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
//pub mod day_22;
pub mod day_23;
pub mod day_24;

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
        (15, Box::new(day_15::Day15 {})),
        (16, Box::new(day_16::Day16 {})),
        (17, Box::new(day_17::Day17 {})),
        (18, Box::new(day_18::Day18 {})),
        (19, Box::new(day_19::Day19 {})),
        (20, Box::new(day_20::Day20 {})),
        (21, Box::new(day_21::Day21 {})),
        //(22, Box::new(day_22::Day22 {})),
        (23, Box::new(day_23::Day23 {})),
        (24, Box::new(day_24::Day24 {})),
    ];

    associations.into_iter().collect()
}
