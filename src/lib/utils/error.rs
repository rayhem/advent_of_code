use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("Attempted to use iterator after exhausting it")]
    WrongSizeIterator,

    #[error("Failed to parse value")]
    ParseIntError(std::num::ParseIntError),

    /// Error indicating a call to [`str::lines`] has yielded nothing. Often
    /// used in [Advent of Code](https://adventofcode.com) problems when the
    /// first line of an input indicates something about the other lines (such
    /// as their length).
    ///
    /// # Examples
    ///
    /// `s.lines().next()` returns `None` if `s == ""`:
    /// ```
    /// use advent::utils::error::Error::EmptyInputStr;
    ///
    /// assert_eq!("".lines().next().ok_or(EmptyInputStr), Err(EmptyInputStr));
    /// assert_eq!("a".lines().next().ok_or(EmptyInputStr), Ok("a"));
    /// ```
    #[error("Expected string to contain at least one line")]
    EmptyInputStr,
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}
