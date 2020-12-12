use crate::solutions::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day11 {}

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(run_simulation(input, 4, Some(1)).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(run_simulation(input, 5, None).to_string())
    }
}

fn run_simulation(input: &str, neighbor_threshold: i32, neighbor_radius: Option<i32>) -> i32 {
    let mut sim = GridSimulation::new(input, neighbor_threshold);

    while sim.state != SimulationState::Finished {
        sim.step(neighbor_radius);
    }

    sim.num_occupied()
}

type Grid = Vec<Vec<GridState>>;

#[derive(Clone, Copy, Debug, PartialEq)]
enum GridState {
    Empty,
    Occupied,
    Floor,
}

impl FromStr for GridState {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, crate::Error> {
        match s {
            "L" => Ok(GridState::Empty),
            "#" => Ok(GridState::Occupied),
            "." => Ok(GridState::Floor),
            _ => Err(crate::Error::BadInput),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SimulationState {
    NotStarted,
    Running,
    Finished,
}

#[derive(Clone)]
struct GridSimulation {
    grid: Grid,
    bounds: (i32, i32),
    state: SimulationState,
    neighbor_threshold: i32,
}

impl GridSimulation {
    fn new(s: &str, neighbor_threshold: i32) -> Self {
        let g: Grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<GridState>().unwrap())
                    .collect()
            })
            .collect();

        GridSimulation {
            bounds: (g.len() as i32, g[0].len() as i32),
            grid: g,
            state: SimulationState::NotStarted,
            neighbor_threshold,
        }
    }

    fn step(&mut self, neighbor_radius: Option<i32>) {
        let mut tmp = self.grid.clone();

        let mut continue_sim = false;
        for (row, col) in (0..self.bounds.0).cartesian_product(0..self.bounds.1) {
            match self.grid[row as usize][col as usize] {
                GridState::Floor => {}
                GridState::Empty => {
                    if let 0 = self.num_occupied_neighbors((row, col), neighbor_radius) {
                        tmp[row as usize][col as usize] = GridState::Occupied;
                        continue_sim = true;
                    }
                }
                GridState::Occupied => {
                    match self.num_occupied_neighbors((row, col), neighbor_radius) {
                        n if (n >= self.neighbor_threshold) => {
                            tmp[row as usize][col as usize] = GridState::Empty;
                            continue_sim = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.state = match continue_sim {
            true => {
                self.grid = tmp;
                SimulationState::Running
            }
            false => SimulationState::Finished,
        };
    }

    fn num_occupied_neighbors(&self, (row, col): (i32, i32), max_distance: Option<i32>) -> i32 {
        let mut neighbors = 0;

        let steps = [-1, 0, 1];
        for (dr, dc) in steps.iter().cartesian_product(steps.iter()) {
            if dr == &0 && dc == &0 {
                continue;
            }

            let mut radius = 1;

            while max_distance.map(|d| radius <= d).unwrap_or(true) {
                let neighbor_row = row + radius * dr;
                if neighbor_row < 0 || neighbor_row >= self.bounds.0 as i32 {
                    break;
                }

                let neighbor_col = col + radius * dc;
                if neighbor_col < 0 || neighbor_col >= self.bounds.1 as i32 {
                    break;
                }

                match self.grid[neighbor_row as usize][neighbor_col as usize] {
                    GridState::Floor => {}
                    GridState::Empty => {
                        break;
                    }
                    GridState::Occupied => {
                        neighbors += 1;
                        break;
                    }
                }

                radius += 1;
            }
        }

        neighbors
    }

    fn num_occupied(&self) -> i32 {
        (&self.grid)
            .iter()
            .flatten()
            .map(|s| match s {
                GridState::Occupied => 1,
                _ => 0,
            })
            .sum()
    }
}
