mod solutions;
use solutions::*;

fn main() {
    let day01 = day01::Day01 {};
    println!(
        "Day 01: {:?}",
        day01.run(include_str!("../inputs/day01.txt"))
    );
}
