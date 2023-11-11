use crate::utils::solution::Solution;

pub struct Day14 {}

const RACE_TIME: i32 = 2503;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(
            input
                .lines()
                .map(|l| parse_input(l).distance_after(RACE_TIME))
                .max()
                .unwrap()
                .to_string(),
        )
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let reindeer = input.lines().map(parse_input).collect::<Vec<_>>();

        Some(
            (1..=RACE_TIME)
                .fold(vec![0; reindeer.len()], |mut acc, time| {
                    let distances = reindeer
                        .iter()
                        .map(|r| r.distance_after(time))
                        .collect::<Vec<_>>();
                    let max_distance = *distances.iter().max().unwrap();

                    for (i, _) in distances
                        .iter()
                        .enumerate()
                        .filter(|(_, d)| **d == max_distance)
                    {
                        acc[i] += 1;
                    }
                    acc
                })
                .iter()
                .max()
                .unwrap()
                .to_string(),
        )
    }
}

struct Reindeer {
    speed: i32,
    duration: i32,
    rest: i32,
}

impl Reindeer {
    fn distance_after(&self, time: i32) -> i32 {
        let interval = self.duration + self.rest;
        let num_intervals = time / interval;
        let remainder = time % interval;
        self.speed * (self.duration * num_intervals + self.duration.min(remainder))
    }
}

fn parse_input(s: &str) -> Reindeer {
    let mut tokens = s.split_whitespace();

    Reindeer {
        speed: tokens.nth(3).unwrap().parse().unwrap(),
        duration: tokens.nth(2).unwrap().parse().unwrap(),
        rest: tokens.nth(6).unwrap().parse().unwrap(),
    }
}

crate::verify!(Day14, crate::my_input!("2015", "14"), "2640", "1102");
