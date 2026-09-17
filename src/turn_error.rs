use thiserror::Error;

/// An error relating to the turning of object.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("no valid turn can be derived")]
pub struct TurnError;
