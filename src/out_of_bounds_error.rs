use crate::AbsoluteDirection;
use crate::Coordinate;
use crate::Positioned;
use thiserror::Error;

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
