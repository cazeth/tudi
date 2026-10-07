use crate::AbsoluteDirection;
use crate::Coordinate;
use crate::Positioned;
use thiserror::Error;

/// An error indicating that an operation would result in an out-of-bounds position.
///
/// There are at least two situations when this might happen: the most typical one is when there is a user-defined boundary,
/// for instance with a [`Grid`](crate::Grid) or a [`BoundedMovingObject`](crate::BoundedMovingObject).
/// These both allow the user to define a [`Bounded`](crate::Bounded) region, which is then upheld
/// during moves and other operations. If the user attempts an operation that would result in an
/// out-of-bounds condition, this error is returned. The other situation is when an operation would result in a
/// position larger or smaller than the library's defined capacity. This might happen when a user tries to add a
/// [`AxisLength`](crate::AxisLength) to a [`Positioned`](crate::Positioned) or add two [`AxisCount`](crate::AxisCount)s to each other that together would exceed the maximum capacity. There may, however, be functions in this library that choose to panic instead of returning an out-of-bounds when an operation would exceed the capacity, because it results in a simpler API.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[error(
    "{} is out of bounds to the {}{}",
    .position,
    .first_out_of_bounds_direction,
    second_direction_suffix(*.second_out_of_bounds_direction)
)]
pub struct OutOfBoundsError {
    position: Coordinate,
    first_out_of_bounds_direction: AbsoluteDirection,
    second_out_of_bounds_direction: Option<AbsoluteDirection>,
}

impl OutOfBoundsError {
    pub fn new<C: Positioned>(
        position: C,
        first_out_of_bounds_direction: AbsoluteDirection,
        second_out_of_bounds_direction: Option<AbsoluteDirection>,
    ) -> Self {
        Self {
            position: *position.position(),
            first_out_of_bounds_direction,
            second_out_of_bounds_direction,
        }
    }

    pub fn position(&self) -> Coordinate {
        self.position
    }

    pub fn first_out_of_bounds_direction(&self) -> AbsoluteDirection {
        self.first_out_of_bounds_direction
    }

    pub fn second_out_of_bounds_direction(&self) -> Option<AbsoluteDirection> {
        self.second_out_of_bounds_direction
    }
}

fn second_direction_suffix(direction: Option<AbsoluteDirection>) -> String {
    direction.map_or_else(String::new, |direction| format!(" and {direction}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let error = OutOfBoundsError::new(
            Coordinate { x: 1, y: -2 },
            AbsoluteDirection::North,
            Some(AbsoluteDirection::East),
        );

        assert_eq!(
            error.to_string(),
            "(1, -2) is out of bounds to the North and East"
        );
    }
}
