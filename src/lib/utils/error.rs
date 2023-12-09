use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Attempted to use iterator after exhausting it")]
    WrongSizeIterator,

    #[error("Failed to parse value")]
    ParseIntError(std::num::ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}
