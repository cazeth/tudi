use thiserror::Error;

#[derive(Debug, Error)]
#[error("invalid input to create Grid")]
pub struct GridCreationError {}
