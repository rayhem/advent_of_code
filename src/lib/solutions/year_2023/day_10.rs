use crate::utils::solution::Solution;
use core::fmt;
use std::str::FromStr;

pub struct Day10 {}

impl Solution for Day10 {
    fn part_one(&self, _input: &str) -> Option<String> {
        None
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        None
    }
}

enum Segment {
    EastToWest,
    NorthToEast,
    NorthToSouth,
    NorthToWest,
    SouthToEast,
    SouthToWest,
    Space,
    Start,
}

#[allow(dead_code)]
impl Segment {
    fn connects_north(&self) -> bool {
        matches!(
            self,
            Self::NorthToEast | Self::NorthToSouth | Self::NorthToWest
        )
    }

    fn connects_west(&self) -> bool {
        matches!(
            self,
            Self::EastToWest | Self::NorthToWest | Self::SouthToWest
        )
    }

    fn connects_east(&self) -> bool {
        matches!(
            self,
            Self::EastToWest | Self::NorthToEast | Self::SouthToEast
        )
    }

    fn connects_south(&self) -> bool {
        matches!(
            self,
            Self::SouthToEast | Self::SouthToWest | Self::NorthToSouth
        )
    }
}

impl TryFrom<char> for Segment {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::NorthToSouth,
            '-' => Self::EastToWest,
            'L' => Self::NorthToEast,
            'J' => Self::NorthToWest,
            '7' => Self::SouthToWest,
            'F' => Self::SouthToEast,
            'S' => Self::Start,
            '.' => Self::Space,
            _ => Err("Bad char")?,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Neighbor {
    East(usize),
    North(usize),
    South(usize),
    West(usize),
}

#[allow(dead_code)]
impl Neighbor {
    fn get(&self) -> usize {
        match self {
            Self::East(n) => *n,
            Self::North(n) => *n,
            Self::South(n) => *n,
            Self::West(n) => *n,
        }
    }
}

#[derive(Clone, Debug)]
struct Dimensions {
    rows: usize,
    cols: usize,
}

#[allow(dead_code)]
impl Dimensions {
    fn to_index(&self, (r, c): (usize, usize)) -> usize {
        r * self.cols + c
    }

    fn to_coordinate(&self, idx: usize) -> (usize, usize) {
        let row = idx / self.cols;
        let col = idx % self.cols;

        (row, col)
    }

    fn neighbors(&self, idx: usize) -> Vec<Neighbor> {
        let (r, c) = self.to_coordinate(idx);
        let mut neighbors = Vec::with_capacity(2);

        if r > 0 {
            neighbors.push(Neighbor::North(self.to_index((r - 1, c))));
        }

        if c > 0 {
            neighbors.push(Neighbor::West(self.to_index((r, c - 1))));
        }

        if c + 1 < self.cols {
            neighbors.push(Neighbor::East(self.to_index((r, c + 1))));
        }

        if r + 1 < self.rows {
            neighbors.push(Neighbor::South(self.to_index((r + 1, c))));
        }

        neighbors
    }
}

struct Network {
    dimensions: Dimensions,
    pipes: Vec<Segment>,
}

#[allow(dead_code)]
impl Network {
    fn start(&self) -> Option<usize> {
        self.pipes.iter().position(|s| matches!(s, Segment::Start))
    }

    fn find_length(&self) -> usize {
        let start_idx = self.start().unwrap();
        let mut stack = vec![start_idx];

        while let Some(_site) = stack.pop() {
            todo!()
        }

        todo!()
    }
}

impl fmt::Debug for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.dimensions)?;
        for row in 0..self.dimensions.rows {
            let mut s = String::with_capacity(self.dimensions.cols);
            for col in 0..self.dimensions.cols {
                s.push(match self.pipes[self.dimensions.to_index((row, col))] {
                    Segment::EastToWest => '─',
                    Segment::NorthToEast => '└',
                    Segment::NorthToSouth => '│',
                    Segment::NorthToWest => '┘',
                    Segment::SouthToEast => '┌',
                    Segment::SouthToWest => '┐',
                    Segment::Space => ' ',
                    Segment::Start => 'S',
                });
            }
            writeln!(f, "{}", s)?;
        }

        fmt::Result::Ok(())
    }
}

impl FromStr for Network {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().map(Segment::try_from);

        let rows = s.lines().count();
        let cols = chars.clone().position(|s| matches!(s, Err(_))).unwrap();

        Ok(Self {
            dimensions: Dimensions { rows, cols },
            pipes: chars.filter(Result::is_ok).collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_conversion() {
        let dimensions = Dimensions { rows: 10, cols: 20 };
        for i in 0..200 {
            assert_eq!(i, dimensions.to_index(dimensions.to_coordinate(i)));
        }
    }

    #[test]
    fn example() {
        const INPUT: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";
        let network = Network::from_str(INPUT).unwrap();
        println!("{:?}", network);

        let start_coord = (2, 0);
        assert!(matches!(
            network.pipes[network.dimensions.to_index(start_coord)],
            Segment::Start
        ));

        let neighbors = network
            .dimensions
            .neighbors(network.dimensions.to_index(start_coord));

        println!("{:?}", neighbors);
        for neighbor in neighbors {
            match neighbor {
                Neighbor::North(n) => {
                    println!("North neighbor {}", network.pipes[n].connects_south())
                }
                Neighbor::West(_) => unreachable!(),
                Neighbor::East(n) => println!("East neighbor {}", network.pipes[n].connects_west()),
                Neighbor::South(n) => {
                    println!("South neighbor {}", network.pipes[n].connects_north())
                }
            }
        }
        // assert_eq!(network.find_length(), 8);
    }
}

crate::verify!(Day10, crate::my_input!("2023", "Day10"), "", "");
