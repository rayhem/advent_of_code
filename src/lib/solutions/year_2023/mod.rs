use super::DayAssociations;

pub mod day_01;

pub fn solutions() -> DayAssociations {
    let associations: Vec<(i32, super::BoxedSolution)> = vec![(1, Box::new(day_01::Day01 {}))];

    associations.into_iter().collect()
}
