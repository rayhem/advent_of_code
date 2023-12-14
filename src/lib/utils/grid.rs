use std::str::FromStr;

pub mod error {
    #[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
    pub enum Error {
        #[error("string contains no data")]
        EmptyInput,
        #[error("expected {item} < {max_value} but got {value}")]
        IndexOutOfBounds {
            item: &'static str,
            value: usize,
            max_value: usize,
        },
        #[error("cannot convert {0} to {1}")]
        DisallowedConversion(&'static str, &'static str),
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
/// println!("{:?}", Location::Coordinate(2, 3).as_index(&dimensions));
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
            Self::Index(idx) => {
                dimensions.check_index(*idx)?;
                Ok(self.clone())
            }
            Self::Coordinate(r, c) => {
                dimensions.check_row_and_col(*r, *c)?;
                Ok(Self::Index(r * dimensions.cols + c))
            }
        }
    }

    /// Converts a [`Location`] to its coordinate representation, ensuring the
    /// resulting [`Location::Index`] lies within the specified dimensions.
    pub fn as_coordinate(&self, dimensions: &Dimensions) -> Result<Self> {
        match self {
            Self::Coordinate(r, c) => {
                dimensions.check_row_and_col(*r, *c)?;
                Ok(self.clone())
            }
            Self::Index(idx) => {
                dimensions.check_index(*idx)?;
                Ok(Self::Coordinate(
                    idx / dimensions.cols,
                    idx % dimensions.cols,
                ))
            }
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

    fn check(value: usize, max_value: usize, name: &'static str) -> Result<()> {
        (value < max_value)
            .then_some(())
            .ok_or(error::Error::IndexOutOfBounds {
                item: name,
                value,
                max_value,
            })
    }

    pub fn check_index(&self, index: usize) -> Result<()> {
        Self::check(index, self.max_index(), "index")
    }

    pub fn check_row(&self, row: usize) -> Result<()> {
        Self::check(row, self.rows, "row")
    }

    pub fn check_col(&self, col: usize) -> Result<()> {
        Self::check(col, self.cols, "column")
    }

    pub fn check_row_and_col(&self, row: usize, col: usize) -> Result<()> {
        self.check_row(row)?;
        self.check_col(col)
    }
}

#[allow(dead_code)]
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
            .ok_or(Self::Err::EmptyInput)?
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
