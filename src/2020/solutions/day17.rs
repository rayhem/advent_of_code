use crate::solutions::Solution;
use itertools::iproduct;
use std::{collections::HashSet, ops::RangeInclusive};

pub struct Day17 {}

impl Solution for Day17 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(simulate(parse_input(input), Dimensions::Three).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(simulate(parse_input(input), Dimensions::Four).to_string())
    }
}

fn simulate(mut active_sites: HashSet<Coordinate>, dimensions: Dimensions) -> usize {
    for _ in 0..6 {
        active_sites = evolve(&active_sites, dimensions);
    }

    active_sites.len()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Clone, Copy, Debug)]
enum Dimensions {
    Three,
    Four,
}

impl Coordinate {
    fn new(w: i32, x: i32, y: i32, z: i32) -> Self {
        Coordinate { w, x, y, z }
    }

    fn neighbors(&self, dimensions: Dimensions) -> impl Iterator<Item = Coordinate> + '_ {
        match dimensions {
            Dimensions::Three => self.neighbors_3d(),
            Dimensions::Four => self.neighbors_4d(),
        }
    }

    fn neighbors_3d(&self) -> Box<dyn Iterator<Item = Coordinate> + '_> {
        const DELTAS: RangeInclusive<i32> = -1..=1;
        Box::new(
            iproduct!(DELTAS.clone(), DELTAS.clone(), DELTAS.clone())
                .filter(|(dx, dy, dz)| !((*dx == 0) && (*dy == 0) && (*dz == 0)))
                .map(move |(dx, dy, dz)| Coordinate::new(0, self.x + dx, self.y + dy, self.z + dz)),
        )
    }

    fn neighbors_4d(&self) -> Box<dyn Iterator<Item = Coordinate> + '_> {
        const DELTAS: RangeInclusive<i32> = -1..=1;
        Box::new(
            iproduct!(
                DELTAS.clone(),
                DELTAS.clone(),
                DELTAS.clone(),
                DELTAS.clone()
            )
            .filter(|(dw, dx, dy, dz)| !((*dw == 0) && (*dx == 0) && (*dy == 0) && (*dz == 0)))
            .map(move |(dw, dx, dy, dz)| {
                Coordinate::new(self.w + dw, self.x + dx, self.y + dy, self.z + dz)
            }),
        )
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
                .map(move |(y, _)| Coordinate::new(0_i32, x as i32, y as i32, 0_i32))
        })
        .collect()
}

fn evolve(cubes: &HashSet<Coordinate>, dimensions: Dimensions) -> HashSet<Coordinate> {
    let mut next = HashSet::new();
    for active_cube in cubes {
        let num_active_neighbors = active_cube
            .neighbors(dimensions)
            .map(|neighbor| match cubes.contains(&neighbor) {
                true => 1,
                false => 0,
            })
            .sum::<i32>();

        if num_active_neighbors == 2 || num_active_neighbors == 3 {
            next.insert(*active_cube);
        }
    }

    for neighbor in cubes
        .iter()
        .map(|coord| coord.neighbors(dimensions))
        .flatten()
    {
        let num_active_neighbors = neighbor
            .neighbors(dimensions)
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
