use std::{char::ParseCharError, num::ParseIntError};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub trait Solution {
    fn part_one(&self, input: &str) -> Option<String>;
    fn part_two(&self, input: &str) -> Option<String>;

    fn run(&self, input: &str) -> Vec<Option<String>> {
        vec![self.part_one(input), self.part_two(input)]
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AdventError {
    BadInput,
}

impl From<ParseCharError> for AdventError {
    fn from(_: ParseCharError) -> Self {
        Self::BadInput
    }
}

impl From<ParseIntError> for AdventError {
    fn from(_: ParseIntError) -> Self {
        Self::BadInput
    }
}
