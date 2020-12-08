use crate::solutions::Solution;
use itertools::Itertools;
use std::collections::HashMap;

type Luggage = HashMap<String, Vec<(i32, String)>>;

pub struct Day07 {}

impl Solution for Day07 {
    fn part_one(&self, input: &str) -> Option<String> {
        let luggage: Luggage = input.lines().map(parse_contents).collect();
        let mut count = 0;

        'bags: for children in luggage.values() {
            let mut to_check = children.clone();
            while !to_check.is_empty() {
                if let Some((_, child)) = to_check.pop() {
                    if child == *"shiny gold" {
                        count += 1;
                        continue 'bags;
                    } else {
                        to_check.append(&mut luggage[&child].clone());
                    }
                }
            }
        }

        Some(count.to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let luggage: Luggage = input.lines().map(parse_contents).collect();
        Some((luggage.count_bags(&"shiny gold".to_owned()) - 1).to_string())
    }
}

trait ContainsBags {
    fn count_bags(&self, target: &str) -> usize;
}

impl ContainsBags for Luggage {
    fn count_bags(&self, target: &str) -> usize {
        let children = &self[target];

        1 + children
            .iter()
            .map(|(num, bag)| (*num as usize) * self.count_bags(bag))
            .sum::<usize>()
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
