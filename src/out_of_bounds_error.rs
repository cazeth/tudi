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
    /// A new `OutOfBoundsError`, with a position specifying where the out-of-bounds error
    /// occurred.
    ///
    /// The position must be out of bounds in at least one direction (otherwise it cannot be out of
    /// bounds). The second direction is optional and intended for use when the position is out of bounds in two directions, for instance when a coordinate is out of bounds to the north east.
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

    /// A new `OutOfBoundsError`, out of bounds in a single direction.
    ///
    /// Use this method to create a new error without having to specify a full out-of-bounds
    /// coordinate, only a single direction and a position in that direction.
    pub fn with_single_direction(axis_coordinate: i128, direction: AbsoluteDirection) -> Self {
        let (x, y) = match direction {
            AbsoluteDirection::East => (Some(axis_coordinate), None),
            AbsoluteDirection::West => (Some(axis_coordinate), None),
            AbsoluteDirection::South => (None, Some(axis_coordinate)),
            AbsoluteDirection::North => (None, Some(axis_coordinate)),
        };

        Self {
            x,
            y,
            first_out_of_bounds_direction: direction,
            second_out_of_bounds_direction: None,
        }
    }

    /// Add a second out of bounds direction.
    ///
    /// The error will sort the directions so that the vertical direction is always first, i.e. adding `AbsoluteDirection::North` as a second direction to an error with `AbsoluteDirection::East` as its first direction will swap the directions so that `AbsoluteDirection::North` is the first direction.
    ///
    /// # Panics
    ///
    /// This method panics if the second direction is equal or inverse to the first direction.
    ///
    /// See also [`AbsoluteDirection::inverse`]
    ///
    /// # Examples
    ///
    /// ```
    /// use tudi::OutOfBoundsError;
    /// use tudi::AbsoluteDirection;
    ///
    /// let mut err = OutOfBoundsError::with_single_direction(10, AbsoluteDirection::East);
    /// err.add_second_direction(5, AbsoluteDirection::North);
    /// assert_eq!(err.first_out_of_bounds_direction(), AbsoluteDirection::North);
    /// assert_eq!(err.second_out_of_bounds_direction(), Some(AbsoluteDirection::East));
    ///
    /// ```
    ///
    pub fn add_second_direction(&mut self, axis_coordinate: i128, direction: AbsoluteDirection) {
        match (self.first_out_of_bounds_direction, direction) {
            (first, second) if first == second.inverse() || first == second => {
                panic!(
                    "Cannot be out of bounds to the same direction twice or to the inverse direction!"
                )
            }
            (first, second)
                if first == AbsoluteDirection::North || first == AbsoluteDirection::South =>
            {
                self.second_out_of_bounds_direction = Some(second);
                self.x = Some(axis_coordinate);
            }
            (first, second)
                if first == AbsoluteDirection::East || first == AbsoluteDirection::West =>
            {
                self.second_out_of_bounds_direction = Some(self.first_out_of_bounds_direction);
                self.first_out_of_bounds_direction = second;
                self.y = Some(axis_coordinate);
            }
            (_, _) => {
                unreachable!();
            }
        }
    }

    /// The out-of-bounds position.
    ///
    /// This method returns `None` if the error does not contain an exact position. This typically
    /// happens when the user has created an error with only a direction and position along a single
    /// axis (leaving the coordinate along the other axis ambiguous). This method also returns `None` if the x- or y-coordinate is greater or smaller than [`Coordinate::MAX`] or [`Coordinate::MIN`].
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

    #[test]
    fn direction_swap() {
        let mut error = OutOfBoundsError::with_single_direction(10, AbsoluteDirection::East);
        error.add_second_direction(-5, AbsoluteDirection::South);
        assert_eq!(
            error.first_out_of_bounds_direction(),
            AbsoluteDirection::South
        );
        assert_eq!(
            error.second_out_of_bounds_direction(),
            Some(AbsoluteDirection::East)
        );
        dbg!(&error);
        assert_eq!(error.position().unwrap().x_coordinate(), 10);
        assert_eq!(error.position().unwrap().y_coordinate(), -5);
    }

    #[test]
    fn single_direction() {
        let error = OutOfBoundsError::with_single_direction(1, AbsoluteDirection::North);
        assert_eq!(error.x, None);
        assert_eq!(error.y, Some(1));
        assert_eq!(
            error.first_out_of_bounds_direction(),
            AbsoluteDirection::North
        );
        assert_eq!(error.second_out_of_bounds_direction, None);
    }

    #[test]
    #[should_panic]
    fn inverse_direction() {
        let mut error = OutOfBoundsError::with_single_direction(1, AbsoluteDirection::East);
        error.add_second_direction(1, AbsoluteDirection::West);
    }

    #[test]
    #[should_panic]
    fn same_direction() {
        let mut error = OutOfBoundsError::with_single_direction(1, AbsoluteDirection::North);
        error.add_second_direction(1, AbsoluteDirection::North);
    }
}
