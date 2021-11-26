mod solutions;
use advent_utils::solution::Solution;
use std::{collections::HashMap, time::Instant};

fn main() {
    let cli = advent_utils::cli::make_cli();

    let mut solutions: HashMap<i32, Box<dyn Solution>> = HashMap::new();
    solutions.insert(1, Box::new(solutions::day01::Day01 {}));
    solutions.insert(2, Box::new(solutions::day02::Day02 {}));
    solutions.insert(3, Box::new(solutions::day03::Day03 {}));
    solutions.insert(4, Box::new(solutions::day04::Day04 {}));
    solutions.insert(5, Box::new(solutions::day05::Day05 {}));
    solutions.insert(6, Box::new(solutions::day06::Day06 {}));

    for day in advent_utils::cli::get_cli_days(&cli).into_iter() {
        if let Some(solution) = solutions.get(&day) {
            let fname = format!("{}/day{:02}.txt", cli.value_of("root").unwrap(), day);
            let input = std::fs::read_to_string(&fname)
                .unwrap_or_else(|_| panic!("File {} not found", fname));
            let now = Instant::now();
            println!(
                "Day {0} ({2}): {1:?}",
                day,
                solution.run(input.as_str()),
                now.elapsed().as_micros()
            );
        } else {
            println!("Day {}: -- UNIMPLEMENTED --", day);
        }
    }
}
