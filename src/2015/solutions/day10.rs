use itertools::Itertools;
use utils::solution::Solution;

pub struct Day10 {}

impl Solution for Day10 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(look_and_say_length_after(input.trim().to_string(), 40).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(look_and_say_length_after(input.trim().to_string(), 50).to_string())
    }
}

fn look_and_say(s: &str) -> String {
    s.chars()
        .dedup_with_count()
        .fold(String::new(), |s, (count, ch)| {
            s + &count.to_string() + &ch.to_string()
        })
}

fn look_and_say_length_after(mut s: String, n: i32) -> usize {
    for _ in 0..n {
        s = look_and_say(&s);
    }

    s.len()
}
