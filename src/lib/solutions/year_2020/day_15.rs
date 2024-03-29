use crate::utils::solution::Solution;
use std::collections::HashMap;

pub struct Day15 {}

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(sequence_at(input, 2020).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(sequence_at(input, 30_000_000).to_string())
    }
}

fn sequence_at(input: &str, n: usize) -> usize {
    let mut game = MemoryGame::new(parse_input(input));

    for _ in 0..n {
        game.take_turn();
    }

    *game.sequence.get(n - 1).unwrap()
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').filter_map(|n| n.parse().ok()).collect()
}

#[derive(Clone, Debug)]
struct MemoryGame {
    sequence: Vec<usize>,
    history: HashMap<usize, usize>,
}

impl MemoryGame {
    fn new(starting_numbers: Vec<usize>) -> Self {
        let mut history: HashMap<_, _> = starting_numbers
            .iter()
            .enumerate()
            .map(|(i, &n)| (n, i))
            .collect();
        history.remove(starting_numbers.last().unwrap());
        MemoryGame {
            history,
            sequence: starting_numbers,
        }
    }

    fn take_turn(&mut self) {
        let prev_turn = self.sequence.len() - 1;
        let prev = *self.sequence.last().unwrap();

        let next = match self.history.get(&prev) {
            None => 0,
            Some(&n) => prev_turn - n,
        };

        self.history.insert(prev, prev_turn);
        self.sequence.push(next);
    }
}
