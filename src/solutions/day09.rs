use advent_utils::solution::Solution;

pub struct Day09 {}

impl Solution for Day09 {
    fn part_one(&self, input: &str) -> Option<String> {
        distance_matrix(input);
        todo!()
    }

    fn part_two(&self, _input: &str) -> Option<String> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
struct LocationPair<'a> {
    from: &'a str,
    to: &'a str,
    distance: i32,
}

impl<'a> TryFrom<&'a str> for LocationPair<'a> {
    type Error = Box<dyn std::error::Error>;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut tokens = s.split_ascii_whitespace();

        Ok(LocationPair {
            from: tokens.nth(0).ok_or("Length error")?,
            to: tokens.nth(1).ok_or("Length error")?,
            distance: tokens
                .clone()
                .nth(1)
                .ok_or("length error")?
                .parse::<i32>()?,
        })
    }
}

fn distance_matrix(_input: &str) -> () {
    todo!()
}

#[cfg(test)]
mod tests {
    mod part1 {
        use super::super::*;

        #[test]
        fn example0() {}
    }

    mod part2 {
        use super::super::*;

        #[test]
        fn example0() {}
    }
}
