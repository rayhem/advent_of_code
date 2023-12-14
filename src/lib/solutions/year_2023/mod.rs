use super::DayAssociations;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;

pub mod day_13;

pub fn solutions() -> DayAssociations {
    let associations: Vec<(i32, super::BoxedSolution)> = vec![
        (1, Box::new(day_01::Day01 {})),
        (2, Box::new(day_02::Day02 {})),
        (3, Box::new(day_03::Day03 {})),
        (4, Box::new(day_04::Day04 {})),
        (6, Box::new(day_06::Day06 {})),
        (7, Box::new(day_07::Day07 {})),
        (8, Box::new(day_08::Day08 {})),
        (9, Box::new(day_09::Day09 {})),
        (10, Box::new(day_10::Day10 {})),
        (11, Box::new(day_11::Day11 {})),
        (13, Box::new(day_13::Day13 {})),
    ];

    associations.into_iter().collect()
}
