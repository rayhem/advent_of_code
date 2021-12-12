use advent_utils::solution::Solution;
use itertools::Itertools;
use std::str::FromStr;

pub struct Day11 {}

impl Solution for Day11 {
    fn part_one(&self, input: &str) -> Option<String> {
        OctopusGarden::from_str(input)
            .map(|mut g| {
                g.update_n(100);
                g.flash_count.to_string()
            })
            .ok()
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let mut garden = OctopusGarden::from_str(input).unwrap();
        let mut i = 0;
        loop {
            i += 1;

            garden.update();
            if garden.data.iter().all(|&value| value == 0) {
                return Some(i.to_string());
            }
        }
    }
}

#[derive(Clone)]
struct OctopusGarden {
    data: Vec<i32>,
    num_rows: usize,
    num_cols: usize,
    flash_count: i32,
}

impl OctopusGarden {
    fn coord_to_idx(&self, (r, c): (usize, usize)) -> usize {
        r * self.num_cols + c
    }

    fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
        (idx / self.num_cols, idx % self.num_cols)
    }

    fn cardinal_neighbors(&self, idx: usize) -> impl Iterator<Item = usize> + '_ + Clone {
        let (r, c) = self.idx_to_coord(idx);

        [
            r.gt(&0).then(|| (r - 1, c)),
            c.gt(&0).then(|| (r, c - 1)),
            c.lt(&(self.num_cols - 1)).then(|| (r, c + 1)),
            r.lt(&(self.num_rows - 1)).then(|| (r + 1, c)),
        ]
        .into_iter()
        .flatten()
        .map(|pair| self.coord_to_idx(pair))
    }

    fn diagonal_neighbors(&self, idx: usize) -> impl Iterator<Item = usize> + '_ + Clone {
        let (r, c) = self.idx_to_coord(idx);
        [
            (r.gt(&0) && c.gt(&0)).then(|| (r - 1, c - 1)),
            (r.gt(&0) && c.lt(&(self.num_cols - 1))).then(|| (r - 1, c + 1)),
            (r.lt(&(self.num_rows - 1)) && c.gt(&0)).then(|| (r + 1, c - 1)),
            (r.lt(&(self.num_rows - 1)) && c.lt(&(self.num_cols - 1))).then(|| (r + 1, c + 1)),
        ]
        .into_iter()
        .flatten()
        .map(|pair| self.coord_to_idx(pair))
    }

    fn all_neighbors(&self, idx: usize) -> impl Iterator<Item = usize> + '_ + Clone {
        self.cardinal_neighbors(idx)
            .chain(self.diagonal_neighbors(idx))
    }

    fn update(&mut self) -> &Self {
        self.data.iter_mut().for_each(|val| *val += 1);

        let mut stack: Vec<usize> = self
            .data
            .iter()
            .enumerate()
            .filter(|(_, &val)| val > 9)
            .map(|(i, _)| i)
            .collect();

        let mut i = 0;
        while i != stack.len() {
            let idx = stack.get(i).unwrap();

            let neighbors = self.all_neighbors(*idx).collect_vec();
            neighbors
                .iter()
                .for_each(|neighbor| self.data[*neighbor] += 1);

            let extension = neighbors
                .iter()
                .filter(|&neighbor| self.data[*neighbor] > 9 && !stack.contains(neighbor))
                .collect_vec();

            stack.extend(extension);

            self.flash_count += 1;
            i += 1;
        }

        self.data
            .iter_mut()
            .filter(|val| **val > 9)
            .for_each(|val| {
                *val = 0;
            });

        self
    }

    fn update_n(&mut self, n: usize) -> &mut Self {
        (0..n).for_each(|_| {
            self.update();
        });
        self
    }
}

impl FromStr for OctopusGarden {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<i32> = s
            .lines()
            .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32))
            .collect();

        Ok(Self {
            data: v,
            num_rows: s.lines().count(),
            num_cols: s.lines().next().ok_or("Invalid input")?.len(),
            flash_count: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

        #[test]
        fn example1() {
            let mut garden = OctopusGarden::from_str(DATA).unwrap();
            (0..100).for_each(|_| {
                garden.update();
            });

            assert_eq!(garden.flash_count, 1656);
        }

        #[test]
        fn example2() {}
    }

    mod integration {
        use super::*;
        const SOLUTION: Day11 = Day11 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day11.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("1694")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("346")));
        }
    }
}
