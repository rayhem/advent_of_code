use super::DayAssociations;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;

pub mod day_06;
pub mod day_07;

pub fn solutions() -> DayAssociations {
    let associations: Vec<(i32, super::BoxedSolution)> = vec![
        (1, Box::new(day_01::Day01 {})),
        (2, Box::new(day_02::Day02 {})),
        (3, Box::new(day_03::Day03 {})),
        (4, Box::new(day_04::Day04 {})),
        (6, Box::new(day_06::Day06 {})),
        (7, Box::new(day_07::Day07 {})),
    ];

    associations.into_iter().collect()
}
