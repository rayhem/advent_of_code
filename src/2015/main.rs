use std::collections::HashMap;
use utils::solution::*;

mod solutions;

fn main() {
    let cli = utils::cli::make_cli();

    let mut solutions: HashMap<i32, Box<dyn Solution>> = HashMap::new();
    solutions.insert(1, Box::new(solutions::day01::Day01 {}));
    solutions.insert(2, Box::new(solutions::day02::Day02 {}));
    solutions.insert(3, Box::new(solutions::day03::Day03 {}));
    solutions.insert(4, Box::new(solutions::day04::Day04 {}));
    solutions.insert(5, Box::new(solutions::day05::Day05 {}));
    solutions.insert(6, Box::new(solutions::day06::Day06 {}));
    solutions.insert(9, Box::new(solutions::day09::Day09 {}));
    solutions.insert(10, Box::new(solutions::day10::Day10 {}));
    solutions.insert(14, Box::new(solutions::day14::Day14 {}));
    solutions.insert(15, Box::new(solutions::day15::Day15 {}));
    solutions.insert(16, Box::new(solutions::day16::Day16 {}));

    let root_dir = cli.value_of("inputs").unwrap();
    for day in utils::cli::get_cli_days(&cli).into_iter() {
        if let Some(solution) = solutions.get(&day) {
            execute_with_timing(day, &format!("{}/day{:02}.txt", root_dir, day), solution);
        }
    }
}
