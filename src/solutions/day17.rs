use crate::solutions::Solution;
use itertools::{iproduct, Itertools, MultiProduct};
use std::{collections::HashSet, iter::Map, ops::RangeInclusive};

pub struct Day17 {}

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> Option<String> {
        let mut universe = parse_input(input);
        for _ in 0..6 {
            universe = evolve(&universe);
        }

        Some(universe.len().to_string())
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coordinate { x, y, z }
    }

    fn neighbors(&self) -> impl Iterator<Item = Coordinate> + '_ {
        const DELTAS: RangeInclusive<i32> = -1..=1;
        iproduct!(DELTAS.clone(), DELTAS.clone(), DELTAS.clone())
            .filter(|(dx, dy, dz)| !((*dx == 0) && (*dy == 0) && (*dz == 0)))
            .map(move |(dx, dy, dz)| Coordinate::new(self.x + dx, self.y + dy, self.z + dz))
    }
}

fn parse_input(input: &str) -> HashSet<Coordinate> {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars()
                .enumerate()
                .filter(move |(_, ch)| *ch == '#')
                .map(move |(y, _)| Coordinate::new(x as i32, y as i32, 0_i32))
        })
        .collect()
}

fn evolve(cubes: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut next = HashSet::new();
    for active_cube in cubes {
        let num_active_neighbors = active_cube
            .neighbors()
            .map(|neighbor| match cubes.get(&neighbor) {
                Some(_) => 1,
                None => 0,
            })
            .sum::<i32>();

        if num_active_neighbors == 2 || num_active_neighbors == 3 {
            next.insert(*active_cube);
        }
    }

    for neighbor in cubes.iter().map(|coord| coord.neighbors()).flatten() {
        let num_active_neighbors = neighbor
            .neighbors()
            .map(|neighbor| match cubes.get(&neighbor) {
                Some(_) => 1,
                None => 0,
            })
            .sum::<i32>();

        if num_active_neighbors == 3 {
            next.insert(neighbor);
        }
    }

    next
}
