use advent_utils::solution::Solution;
use itertools::Itertools;
use std::{collections::HashSet, ops::Deref, str::FromStr};

pub struct Day09 {}

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> Option<String> {
        let grid = Grid::from_str(input).unwrap();
        Some(grid.count_minima().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let grid = Grid::from_str(input).unwrap();
        Some(grid.basin_size().to_string())
    }
}

#[derive(Clone, Copy, Debug)]
enum Site {
    Unexamined(i32),
    Basin(i32, i32),
    Maximum,
}

impl Site {
    fn value(self) -> i32 {
        match self {
            Self::Unexamined(n) => n,
            Self::Basin(n, _) => n,
            Self::Maximum => 9,
        }
    }
}

impl<T> From<T> for Site
where
    T: Deref<Target = i32>,
{
    fn from(i: T) -> Self {
        match *i {
            9 => Self::Maximum,
            n => Self::Unexamined(n),
        }
    }
}

#[derive(Clone, Debug)]
struct Grid {
    data: Vec<Site>,
    num_rows: usize,
    num_cols: usize,
}

impl Grid {
    fn coord_to_idx(&self, (r, c): (usize, usize)) -> usize {
        r * self.num_cols + c
    }

    fn idx_to_coord(&self, idx: usize) -> (usize, usize) {
        (idx / self.num_cols, idx % self.num_cols)
    }

    fn adjacent_neighbors(&self, idx: usize) -> impl Iterator<Item = usize> + '_ {
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

    fn adjacent_values(&self, idx: usize) -> impl Iterator<Item = Site> + '_ {
        self.adjacent_neighbors(idx).map(|i| self.data[i])
    }

    fn count_minima(&self) -> i32 {
        self.data
            .iter()
            .enumerate()
            .filter(|(i, minimal_site)| {
                self.adjacent_values(*i)
                    .all(|site| minimal_site.value() < site.value())
            })
            .map(|(_, site)| 1 + site.value())
            .sum()
    }

    fn neighbors(&self, idx: usize) -> impl Iterator<Item = usize> + '_ {
        let mut queue: Vec<_> = vec![idx];
        let mut indices: HashSet<usize> = HashSet::new();

        while let Some(site) = queue.pop() {
            indices.insert(site);
            queue.extend(self.adjacent_neighbors(site).filter(|neighbor| {
                !matches!(self.data[*neighbor], Site::Maximum) && !indices.contains(neighbor)
            }))
        }

        indices.into_iter()
    }

    fn basin_size(mut self) -> i32 {
        let mut idx: usize = 0;
        let mut basin_idx = 0;
        while idx < self.data.len() {
            if let Site::Unexamined(n) = self.data[idx] {
                let neighbors = self.neighbors(idx).collect_vec();
                neighbors
                    .into_iter()
                    .for_each(|neighbor| self.data[neighbor] = Site::Basin(n, basin_idx));
                basin_idx += 1
            }
            idx += 1;
        }

        let mut basin_sizes = self
            .data
            .into_iter()
            .filter_map(|b| {
                if let Site::Basin(_, basin_id) = b {
                    Some(basin_id)
                } else {
                    None
                }
            })
            .counts()
            .into_iter()
            .collect_vec();
        basin_sizes.sort_unstable_by(|(_, size1), (_, size2)| size2.cmp(size1));

        basin_sizes
            .into_iter()
            .take(3)
            .map(|(_, size)| size as i32)
            .product()
    }
}

impl FromStr for Grid {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Grid {
            data: s
                .replace('\n', "")
                .chars()
                .flat_map(|c| Site::try_from(&(c.to_digit(10).unwrap() as i32)))
                .collect(),
            num_rows: s.lines().count(),
            num_cols: s.lines().next().ok_or("Invalid input")?.len(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod unit {
        use super::*;
        static DATA: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

        #[test]
        fn grid_functionality() {
            let grid = Grid::from_str(DATA).unwrap();
            assert_eq!(grid.num_rows, 5);
            assert_eq!(grid.num_cols, 10);

            for i in 0..grid.data.len() {
                assert_eq!(grid.coord_to_idx(grid.idx_to_coord(i)), i)
            }
        }

        #[test]
        fn example1() {
            let grid = Grid::from_str(DATA).unwrap();
            assert_eq!(grid.count_minima(), 15);
        }

        #[test]
        fn example2() {
            let grid = Grid::from_str(DATA).unwrap();
            assert_eq!(grid.basin_size(), 1134);
        }
    }

    mod integration {
        use super::*;
        const SOLUTION: Day09 = Day09 {};
        static INPUT: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/day09.dat"));

        #[test]
        fn part_one() {
            assert_eq!(SOLUTION.part_one(INPUT), Some(String::from("480")));
        }

        #[test]
        fn part_two() {
            assert_eq!(SOLUTION.part_two(INPUT), Some(String::from("1045660")));
        }
    }
}
