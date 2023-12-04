use super::DayAssociations;

pub mod day_01;
pub mod day_02;
pub mod day_03;

pub fn solutions() -> DayAssociations {
    let associations: Vec<(i32, super::BoxedSolution)> = vec![
        (1, Box::new(day_01::Day01 {})),
        (2, Box::new(day_02::Day02 {})),
        (3, Box::new(day_03::Day03 {})),
    ];

    associations.into_iter().collect()
}
