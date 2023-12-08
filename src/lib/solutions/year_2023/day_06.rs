use crate::utils::solution::Solution;
use itertools::Itertools;

pub struct Day06 {}

impl Solution for Day06 {
    fn part_one(&self, input: &str) -> Option<String> {
        let races = parse_part1(input);
        Some(races.iter().map(Race::solve).product::<u64>().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let race = parse_part2(input);
        Some(race.solve().to_string())
    }
}

fn parse_part1(s: &str) -> Vec<Race> {
    let mut lines = s.lines();

    let times = lines
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, times)| times.split_whitespace().map(str::parse::<u64>))
        .unwrap();

    let distances = lines
        .next()
        .unwrap()
        .split_once(':')
        .map(|(_, distances)| distances.split_whitespace().map(str::parse::<u64>))
        .unwrap();

    times
        .zip(distances)
        .map(|(t, d)| Race {
            duration: t.unwrap(),
            distance: d.unwrap(),
        })
        .collect()
}

fn parse_part2(s: &str) -> Race {
    let (time, distance) = s
        .lines()
        .map(|line| {
            line.split_once(':')
                .map(|(_, ts)| {
                    ts.split_ascii_whitespace()
                        .fold(String::new(), |acc, token| acc + token)
                        .parse::<u64>()
                        .unwrap()
                })
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    Race {
        duration: time,
        distance,
    }
}

#[derive(Clone, Debug)]
struct Race {
    duration: u64,
    distance: u64,
}

impl Race {
    /// Computes the number of integers that satisfy the race's record
    /// condition, namely that boat_distance(charge_time, race_time) > record
    /// distance. Expanding this out gives charge_time ^ 2 - race_time *
    /// charge_time + record_distance < 0 which is easily solved with the
    /// quadratic equation. The resulting number of ways to set the record is
    /// just the number of integers between the roots.
    fn solve(&self) -> u64 {
        let discriminant = ((self.duration * self.duration - 4 * self.distance) as f64).sqrt();
        let upper_root = (-(self.duration as f64) + discriminant) / 2.0;
        let lower_root = (-(self.duration as f64) - discriminant) / 2.0;

        (upper_root.ceil() - lower_root.floor()) as u64 - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";
        let races = parse_part1(INPUT);

        assert_eq!(races[0].solve(), 4);
        assert_eq!(races[1].solve(), 8);
        assert_eq!(races[2].solve(), 9);

        let megarace = parse_part2(INPUT);

        assert_eq!(megarace.solve(), 71503);
    }
}

crate::verify!(
    Day06,
    crate::my_input!("2023", "Day06"),
    "861300",
    "28101347"
);
