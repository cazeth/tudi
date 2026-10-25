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
#[error("{}", self.message())]
pub struct OutOfBoundsError {
    x: Option<i128>,
    y: Option<i128>,
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
            x: Some(i128::from(position.x_coordinate())),
            y: Some(i128::from(position.y_coordinate())),
            first_out_of_bounds_direction,
            second_out_of_bounds_direction,
        }
    }

    pub fn position(&self) -> Option<Coordinate> {
        if let Some(x) = self.x {
            if let Some(y) = self.y {
                Some(Coordinate {
                    x: i32::try_from(x).ok()?,
                    y: i32::try_from(y).ok()?,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn first_out_of_bounds_direction(&self) -> AbsoluteDirection {
        self.first_out_of_bounds_direction
    }

    pub fn second_out_of_bounds_direction(&self) -> Option<AbsoluteDirection> {
        self.second_out_of_bounds_direction
    }

    fn message(&self) -> String {
        let location = match (self.position(), self.x, self.y) {
            (Some(pos), _, _) => format!("({}, {})", pos.x_coordinate(), pos.y_coordinate()),
            (None, Some(x), None) => format!("x: {x}"),
            (None, Some(x), Some(y)) => format!("({}, {})", x, y),
            (None, None, Some(y)) => format!("y: {y}"),
            (None, None, None) => unreachable!(),
        };

        format!(
            "{} is out of bounds to the {}{}",
            location,
            self.first_out_of_bounds_direction,
            second_direction_suffix(self.second_out_of_bounds_direction)
        )
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
