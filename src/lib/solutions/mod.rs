pub mod year_2015;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;

type BoxedSolution = Box<dyn crate::utils::solution::Solution>;
type DayAssociations = std::collections::HashMap<i32, BoxedSolution>;
type YearAssociations = std::collections::HashMap<i32, DayAssociations>;

pub fn all_solutions() -> YearAssociations {
    let associations: Vec<(i32, DayAssociations)> = vec![(2015, year_2015::solutions())];
    associations.into_iter().collect()
}
