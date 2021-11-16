mod solutions;
use solutions::*;
use std::time::Instant;

fn main() {
    let solutions: Vec<Option<Box<dyn Solution>>> = vec![
        Some(Box::new(day01::Day01 {})),
        Some(Box::new(day02::Day02 {})),
        Some(Box::new(day03::Day03 {})),
        Some(Box::new(day04::Day04 {})),
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
