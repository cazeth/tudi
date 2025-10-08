use crate::Coordinate;
use crate::Positioned;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("{:?} is out of bounds", .position)]
pub struct OutOfBoundsError {
    position: Coordinate,
}

impl OutOfBoundsError {
    pub fn new<C: Positioned>(position: C) -> Self {
        Self {
            position: *position.position(),
        }
    }

    pub fn position(&self) -> Coordinate {
        self.position
    }
}
