use thiserror::Error;

use crate::Coordinate;
use crate::OutOfBoundsError;

/// The main error type for a Grid.
///
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GridError {
    #[error("Out of Bounds")]
    OutOfBoundsError(#[from] OutOfBoundsError),
    #[error("Collision")]
    CollisionError,

    #[error("Unoccupied at {0:?}")]
    UnoccupiedError(Coordinate),
}
