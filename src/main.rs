mod solutions;
use solutions::*;
use std::time::Instant;

fn main() {
    let solutions: Vec<Option<Box<dyn Solution>>> = vec![
        Some(Box::new(day01::Day01 {})),
        Some(Box::new(day02::Day02 {})),
        Some(Box::new(day03::Day03 {})),
        Some(Box::new(day04::Day04 {})),
        Some(Box::new(day05::Day05 {})),
        Some(Box::new(day06::Day06 {})),
        Some(Box::new(day07::Day07 {})),
        Some(Box::new(day08::Day08 {})),
        Some(Box::new(day09::Day09 {})),
        Some(Box::new(day10::Day10 {})),
        Some(Box::new(day11::Day11 {})),
        Some(Box::new(day12::Day12 {})),
        Some(Box::new(day13::Day13 {})),
        Some(Box::new(day14::Day14 {})),
        Some(Box::new(day15::Day15 {})),
        Some(Box::new(day16::Day16 {})),
        Some(Box::new(day17::Day17 {})),
    ];

    for (day, solution) in solutions.iter().enumerate() {
        let day = format!("{:02}", day + 1);

        match solution {
            Some(x) => {
                let fname = format!("inputs/day{}.txt", day);
                let input = std::fs::read_to_string(&fname)
                    .unwrap_or_else(|_| panic!("File {} not found", fname));
                let now = Instant::now();
                println!(
                    "Day {0} ({2}): {1:?}",
                    day,
                    x.run(input.as_str()),
                    now.elapsed().as_micros()
                );
            }
            None => println!("Day {}: -- UNIMPLEMENTED --", day),
        }
    }
}
