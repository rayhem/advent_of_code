mod solutions;
use solutions::*;

fn main() {
    let solutions: Vec<Option<Box<dyn Solution>>> = vec![
        Some(Box::new(day01::Day01 {})),
        Some(Box::new(day02::Day02 {})),
        Some(Box::new(day03::Day03 {})),
        Some(Box::new(day04::Day04 {})),
        Some(Box::new(day05::Day05 {})),
    ];

    for (day, solution) in solutions.iter().enumerate() {
        let day = format!("{:02}", day + 1);

        match solution {
            Some(x) => {
                let fname = format!("inputs/day{}.txt", day);
                let input =
                    std::fs::read_to_string(&fname).expect(&format!("File {} not found", fname));
                println!("Day {}: {:?}", day, x.run(input.as_str()));
            }
            None => println!("Day {}: -- UNIMPLEMENTED --", day),
        }
    }
}
