use std::str::FromStr;

use crate::utils::solution::Solution;

pub struct Day18 {}

impl Solution for Day18 {
    fn part_one(&self, input: &str) -> Option<String> {
        let mut grid = Grid::from_str(input).unwrap();
        for _ in 0..100 {
            grid.update();
        }

        Some(grid.total().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let mut grid = Grid::from_str(input).unwrap();
        for _ in 0..100 {
            grid.set_corners(Light::On);
            grid.update();
        }

        grid.set_corners(Light::On);
        Some(grid.total().to_string())
    }
}

#[derive(Clone, Copy, Debug)]
enum Light {
    On,
    Off,
}

impl From<char> for Light {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::On,
            '.' => Self::Off,
            _ => panic!("Invalid light character"),
        }
    }
}

type Index = usize;

#[derive(Clone, Debug)]
struct Grid {
    size: usize,
    cells: Vec<Light>,
}

impl Grid {
    fn index(&self, row: Index, col: Index) -> Index {
        (row % self.size) * self.size + (col % self.size)
    }

    fn count_neighbors(&self, row: Index, col: Index) -> i32 {
        let mut count = 0;
        for r in (row as isize - 1).max(0)..=(row as isize + 1).min((self.size - 1) as isize) {
            for c in (col as isize - 1).max(0)..=(col as isize + 1).min((self.size - 1) as isize) {
                if r as Index == row && c as Index == col {
                    continue;
                }

                count += match self.cells[self.index(r as Index, c as Index)] {
                    Light::Off => 0,
                    Light::On => 1,
                }
            }
        }

        count
    }

    fn update(&mut self) {
        let mut new_cells = self.cells.clone();
        for col in 0..self.size {
            for row in 0..self.size {
                let idx = self.index(row, col);

                let cell = self.cells[idx];
                let neighbors = self.count_neighbors(row, col);
                new_cells[idx] = match (cell, neighbors) {
                    (Light::On, 2..=3) => Light::On,
                    (Light::Off, 3) => Light::On,
                    _ => Light::Off,
                };
            }
        }
        self.cells = new_cells;
    }

    fn set_corners(&mut self, state: Light) {
        let n = self.size - 1;
        let (a, b, c, d) = (
            self.index(0, 0),
            self.index(0, n),
            self.index(n, 0),
            self.index(n, n),
        );

        self.cells[a] = state;
        self.cells[b] = state;
        self.cells[c] = state;
        self.cells[d] = state;
    }

    fn total(&self) -> i32 {
        self.cells.iter().fold(0, |acc, cell| {
            acc + match cell {
                Light::Off => 0,
                Light::On => 1,
            }
        })
    }
}

impl std::str::FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            size: s.lines().next().unwrap().len(),
            cells: s
                .lines()
                .flat_map(|line| line.chars().map(Light::from))
                .collect(),
        })
    }
}

crate::verify!(Day18, crate::my_input!("2015", "18"), "821", "886");
