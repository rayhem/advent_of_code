use advent_utils::solution::*;
use std::collections::HashMap;

mod solutions;

fn main() {
    let cli = advent_utils::cli::make_cli();

    let mut solutions: HashMap<i32, Box<dyn Solution>> = HashMap::new();
    solutions.insert(1, Box::new(solutions::day01::Day01 {}));
    solutions.insert(2, Box::new(solutions::day02::Day02 {}));
    solutions.insert(3, Box::new(solutions::day03::Day03 {}));
    solutions.insert(4, Box::new(solutions::day04::Day04 {}));
    solutions.insert(5, Box::new(solutions::day05::Day05 {}));
    solutions.insert(6, Box::new(solutions::day06::Day06 {}));
    solutions.insert(7, Box::new(solutions::day07::Day07 {}));

    let root_dir = cli.value_of("inputs").unwrap();
    for day in advent_utils::cli::get_cli_days(&cli).into_iter() {
        if let Some(solution) = solutions.get(&day) {
            execute_with_timing(day, &format!("{}/day{:02}.dat", root_dir, day), solution)
        }
    }
}
