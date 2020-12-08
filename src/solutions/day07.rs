use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;

pub struct Day07 {}

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> Option<String> {
        let bags: HashMap<_, _> = input.lines().map(parse_contents).collect();
        let mut count = 0;

        let target = String::from("shiny gold");

        'bags: for (_, children) in &bags {
            let mut to_check = children.clone();
            while !to_check.is_empty() {
                if let Some((_, child)) = to_check.pop() {
                    if child == target {
                        count += 1;
                        continue 'bags;
                    } else {
                        to_check.append(&mut bags[&child].clone());
                    }
                }
            }
        }

        Some(count.to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

fn parse_contents(s: &str) -> (String, Vec<(i32, String)>) {
    let mut halves = s.split(" bags contain ");
    let (left, right) = (halves.next().unwrap(), halves.next().unwrap());

    let mut contents = Vec::new();
    for (qty, adj, color, _) in right.split_whitespace().tuples() {
        contents.push((qty.parse::<i32>().unwrap(), format!("{} {}", adj, color)));
    }

    (left.to_string(), contents)
}

#[cfg(test)]
mod tests {
    use super::*;
}
