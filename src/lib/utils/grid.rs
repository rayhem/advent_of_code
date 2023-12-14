use std::str::FromStr;

pub mod error {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Error {
        ParseError,
        OutOfBoundsError,
        DisallowedConversion,
    }
}

/// Result type for [`Grid`] operations that can fail
pub type Result<T> = std::result::Result<T, error::Error>;

/// Type-safe representation of a location within a [`Grid`].
///
/// # Examples
///
/// The first `value` is at the top-left corner:
/// ```
/// use advent::utils::grid::*;
///
/// let dimensions = Dimensions::new(10, 10); // A 10-by-10 grid
/// assert_eq!(Location::Index(0).as_coordinate(&dimensions), Ok(Location::Coordinate(0, 0)));
/// assert_eq!(Location::Coordinate(0, 0).as_index(&dimensions), Ok(Location::Index(0)));
/// ```
/// A row-major 5-by-3 grid has the following indices:
/// ```
/// use advent::utils::grid::*;
///
/// // 0   1  2
/// // 3   4  5
/// // 6   7  8
/// // 9  10 11
/// // 12 13 14
/// let dimensions = Dimensions::new(5, 3);
/// assert_eq!(Location::Coordinate(1, 2).as_index(&dimensions), Ok(Location::Index(5)));
/// assert_eq!(Location::Coordinate(2, 3).as_index(&dimensions), Err(error::Error::OutOfBoundsError));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Location {
    /// Index variant. Reprsents offsets from the first value stored for a
    /// [`Grid`]. Space-efficient way to store a collection of [`Location`]s.
    Index(usize),
    /// Coordinate variant. Straightforward for computing neighbors.
    Coordinate(usize, usize),
}

impl Location {
    /// Converts a [`Location`] to its index representation, ensuring the
    /// resulting [`Location::Index`] lies within the specified dimensions.
    pub fn as_index(&self, dimensions: &Dimensions) -> Result<Self> {
        match self {
            Self::Index(idx) => (*idx < dimensions.max_index())
                .then_some(self.clone())
                .ok_or(error::Error::OutOfBoundsError),
            Self::Coordinate(r, c) => (*r < dimensions.rows && *c < dimensions.cols)
                .then_some(Self::Index(r * dimensions.cols + c))
                .ok_or(error::Error::OutOfBoundsError),
        }
    }

    /// Converts a [`Location`] to its coordinate representation, ensuring the
    /// resulting [`Location::Index`] lies within the specified dimensions.
    pub fn as_coordinate(&self, dimensions: &Dimensions) -> Result<Self> {
        match self {
            Self::Coordinate(r, c) => (*r < dimensions.rows && *c < dimensions.cols)
                .then_some(self.clone())
                .ok_or(error::Error::OutOfBoundsError),
            Self::Index(idx) => (*idx < dimensions.max_index())
                .then_some(Self::Coordinate(
                    idx / dimensions.cols,
                    idx % dimensions.cols,
                ))
                .ok_or(error::Error::OutOfBoundsError),
        }
    }

    /// Converts a [`Location`] to its index representation, ignoring bounds
    /// imposed by `dimensions`. A [`Location::Index`] produced this way can
    /// exceed the maximum index allowed by `dimensions`.
    pub fn as_index_unchecked(&self, dimensions: &Dimensions) -> Self {
        match self {
            Self::Index(_) => self.clone(),
            Self::Coordinate(r, c) => Self::Index(r * dimensions.cols + c),
        }
    }

    /// Converts a [`Location`] to its coordinate representation, ignoring
    /// bounds imposed by `dimensions`. a [`Location::Coordinate`] produced this
    /// way can have more rows, more columns, or more rows *and* columns than
    /// what's allowed by `dimensions`.
    pub fn as_coordinate_unchecked(&self, dimensions: &Dimensions) -> Self {
        match self {
            Self::Coordinate(_, _) => self.clone(),
            Self::Index(idx) => Self::Coordinate(idx / dimensions.rows, idx % dimensions.rows),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Dimensions {
    rows: usize,
    cols: usize,
}

impl Dimensions {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }

    pub fn max_index(&self) -> usize {
        self.rows * self.cols
    }
}

#[derive(Clone, Debug, Default)]
pub struct Grid<T> {
    values: Vec<T>,
    dimensions: Dimensions,
}

impl FromStr for Grid<char> {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self> {
        let num_rows = s.lines().count();
        let num_cols = s
            .lines()
            .next()
            .ok_or(Self::Err::ParseError)?
            .chars()
            .count();

        Ok(Self {
            values: s.lines().flat_map(|line| line.chars()).collect(),
            dimensions: Dimensions::new(num_rows, num_cols),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_grid_from_string() {
        const INPUT: &str = "abc\ndef\nghi";
        let char_grid = Grid::from_str(INPUT).unwrap();

        assert_eq!(char_grid.values, ('a'..='i').collect::<Vec<_>>());
        assert_eq!(char_grid.dimensions, Dimensions::new(3, 3));
    }
}
