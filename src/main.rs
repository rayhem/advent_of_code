mod solutions;
use solutions::*;

fn main() {
    let solutions: Vec<Box<dyn Solution>> =
        vec![Box::new(day01::Day01 {}), Box::new(day02::Day02 {})];

    for (day, solution) in solutions.iter().enumerate() {
        let day = format!("{:02}", day + 1);
        let input =
            std::fs::read_to_string(format!("inputs/day{}.txt", day)).expect("Could not open file");
        println!("Day {}: {:?}", day, solution.run(input.as_str()))
    }
}
