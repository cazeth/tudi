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
        first_row_count: usize,
        second_row_index: usize,
        second_row_count: usize,
    },

    #[error("A grid with no coordinates is not allowed.")]
    Empty,
}
