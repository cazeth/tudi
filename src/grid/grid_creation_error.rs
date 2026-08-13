use crate::AxisCountError;
use thiserror::Error;

/// Errors relating to the creation of a [`Grid`](crate::grid::Grid).
///
/// Some grid constructors are fallible; a user may try to create a grid with different rows lengths
/// or try to create a grid with no coordinates. In such cases, this type is returned.
///
/// This enum is marked as non-exhaustive as the api is currently being developed. It is expected to
/// be marked as exhaustive once the api is more stable.
///
/// See also [`GridError`](crate::GridError).
#[derive(Debug, Clone, Error, PartialEq, Eq, PartialOrd, Ord)]
#[error("invalid input to create Grid")]
#[non_exhaustive]
pub enum GridCreationError {
    #[error(
        "Rows of different counts in input: found row {first_row_index} of count {first_row_count} and row {second_row_index} of count {second_row_count}"
    )]
    DifferentRowLengths {
        first_row_index: usize,
        first_row_count: u64,
        second_row_index: usize,
        second_row_count: u64,
    },

    #[error("A grid axis cannot contain {count} coordinates.")]
    CountTooLarge { count: u64 },

    #[error("A grid with no coordinates is not allowed.")]
    Empty,
}

impl From<AxisCountError> for GridCreationError {
    fn from(value: AxisCountError) -> Self {
        match value {
            AxisCountError::Zero => GridCreationError::Empty,
            AxisCountError::TooLarge(x) => GridCreationError::CountTooLarge { count: x },
            // There are no grid constructors that take signed integers as arguments so this
            // should never happen.
            AxisCountError::Negative(_) => {
                unreachable!(
                    "Should never be able to try to construct a grid from negative numbers!"
                )
            }
        }
    }
}
