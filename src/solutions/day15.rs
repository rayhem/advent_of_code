use crate::solutions::Solution;
use std::collections::HashMap;

pub struct Day15 {}

impl Solution for Day15 {
    fn part_one(&self, input: &str) -> Option<String> {
        let mut game = MemoryGame::new(parse_input(input));

        for _ in 0..2020 {
            game.take_turn();
        }

        Some(game.sequence.iter().nth(2019).unwrap().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let mut game = MemoryGame::new(parse_input(input));

        for _ in 0..30000000 {
            game.take_turn();
        }

        Some(game.sequence.iter().nth(30000000 - 1).unwrap().to_string())
    }
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
            .map(|(i, &n)| (n, i as usize))
            .collect();
        history.remove(starting_numbers.last().unwrap());
        MemoryGame {
            history: history,
            sequence: starting_numbers,
        }
    }

    fn take_turn(&mut self) {
        let prev_turn = (self.sequence.len() as usize) - 1;
        let prev = *self.sequence.last().unwrap() as usize;

        let next = match self.history.get(&prev) {
            None => 0,
            Some(&n) => prev_turn - n,
        };

        self.history.insert(prev, prev_turn);
        self.sequence.push(next);
    }
}
