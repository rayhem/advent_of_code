use advent_utils::solution::Solution;

pub struct Day14 {}

const MAX_TIME: i32 = 2503;

impl Solution for Day14 {
    fn part_one(&self, input: &str) -> Option<String> {
        input
            .lines()
            .map(Reindeer::try_from)
            .flatten()
            .map(|reindeer| reindeer.distance(MAX_TIME))
            .max()
            .map(|v| v.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let reindeer = input
            .lines()
            .map(Reindeer::try_from)
            .flatten()
            .collect::<Vec<_>>();
        let mut scores = vec![0; reindeer.len()];

        for time in 1..=MAX_TIME {
            let max_distance = reindeer.iter().map(|r| r.distance(time)).max().unwrap();

            for (i, r) in reindeer.iter().enumerate() {
                if r.distance(time) == max_distance {
                    scores[i] += 1;
                }
            }
        }

        Some(scores.iter().max().unwrap().to_string())
    }
}

#[derive(Clone, Copy, Debug)]
struct Reindeer {
    speed: i32,
    fly_duration: i32,
    rest_duration: i32,
}

impl TryFrom<&str> for Reindeer {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tokens = value.split_whitespace();
        Ok(Reindeer {
            speed: tokens.nth(3).unwrap().parse()?,
            fly_duration: tokens.nth(2).unwrap().parse()?,
            rest_duration: tokens.nth(6).unwrap().parse()?,
        })
    }
}

impl Reindeer {
    fn distance(&self, time: i32) -> i32 {
        let interval_duration = self.fly_duration + self.rest_duration;
        let num_intervals = time / interval_duration;
        self.speed
            * (self.fly_duration * num_intervals
                + std::cmp::min(time % interval_duration, self.fly_duration))
    }
}

#[cfg(test)]
mod tests {
    mod part1 {
        use super::super::*;

        #[test]
        fn example0() {}
    }

    mod part2 {
        use super::super::*;

        #[test]
        fn example0() {}
    }
}
