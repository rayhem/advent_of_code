pub mod year_2015;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;
pub mod year_2023;
pub mod year_2024;

type BoxedSolution = Box<dyn crate::utils::solution::Solution>;
type DayAssociations = std::collections::HashMap<i32, BoxedSolution>;
type YearAssociations = std::collections::HashMap<i32, DayAssociations>;

pub fn all_solutions() -> YearAssociations {
    let associations: Vec<(i32, DayAssociations)> = vec![
        (2015, year_2015::solutions()),
        (2023, year_2023::solutions()),
        (2024, year_2024::solutions()),
    ];
    associations.into_iter().collect()
}
