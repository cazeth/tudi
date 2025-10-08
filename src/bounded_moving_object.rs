use crate::AbsoluteDirection;
#[allow(deprecated)]
use crate::Coordinate;
use crate::DynamicallyBounded;
use crate::Mover;
use crate::OutOfBoundsError;
use crate::Positioned;
use crate::RelativeDirection;
use crate::bounded::Bounded;
use crate::bounded::MaybeOriginBounded;
use crate::bounded::MaybeOriginCentered;
use crate::bounded::OriginCenteredness;
use crate::bounds::Bounds;

/// A bounded movable object that occupies a single point.
#[derive(Clone, Debug)]
pub struct BoundedMovingObject {
    current_pos: Coordinate,
    current_direction: AbsoluteDirection,
    bounds: Bounds,
}

impl BoundedMovingObject {
    /// Creates new BoundedMovingObject. If the origin is in bounds, the marker is set to the
    /// origin. If not, it is set to the southeast corner.
    /// input are in the order x_min, x_max, y_min, y_max
    /// # Panics
    ///
    /// This method panics if x_min_boundary > x_max_boundary or y_min_boundary > y_max_boundary
    ///
    /// # Examples
    /// ```
    /// use tudi::BoundedMovingObject;
    /// use tudi::Positioned;
    /// use tudi::Coordinate;
    /// use tudi::Bounds;
    /// let bounds = Bounds::new(-10,20,-10,20);
    /// let mut pos = BoundedMovingObject::from(bounds);
    /// assert_eq!(pos.position(), &Coordinate {x : 0, y : 0});
    ///
    /// let bounds = Bounds::new(5,5,5,5);
    /// let mut pos = BoundedMovingObject::from(bounds);
    /// assert_eq!(pos.position(), &Coordinate {x : 5, y : 10});
    ///
    /// ```
    fn new(
        x_min_boundary: i32,
        x_max_boundary: i32,
        y_min_boundary: i32,
        y_max_boundary: i32,
    ) -> Self {
        assert!(x_max_boundary >= x_min_boundary);
        assert!(y_max_boundary >= y_min_boundary);

        let bounds: Bounds = Bounds::new(
            x_min_boundary,
            (x_max_boundary - x_min_boundary) as usize,
            y_min_boundary,
            (y_max_boundary - y_min_boundary) as usize,
        );

        let mut result = Self {
            current_pos: Coordinate {
                x: x_min_boundary,
                y: y_max_boundary,
            },
            bounds,
            current_direction: AbsoluteDirection::North,
        };

        // if origin is within the bounds, it sets the marker to origin.
        if result.is_within_bounds(&Coordinate::default()) {
            result.current_pos = Coordinate::default();
        };
        result
    }

    pub fn turn_toward<C: Positioned>(&mut self, target: &C) -> Result<&AbsoluteDirection, String> {
        let directions = self.direction_toward(target.position());
        if directions.0 == directions.1 {
            self.current_direction = directions.0;
            Ok(&self.current_direction)
        } else {
            Err("no clean turn.".to_string())
        }
    }

    pub fn turn(&mut self, dir: RelativeDirection) {
        match dir {
            RelativeDirection::Left => {
                self.set_current_direction(self.direction().increment());
            }
            RelativeDirection::Right => {
                self.set_current_direction(self.direction().decrement());
            }
        }
    }

    /// same as move_in_current_direction but reports the new position of the object, which makes
    /// it possible to keep track of what happenend.
    pub fn move_in_current_direction_and_return_new_pos(&mut self, magnitude: usize) -> Coordinate {
        let dir = self.direction();
        self.move_in_absolute_direction(*dir, magnitude);
        self.current_pos
    }

    pub fn get_signed_boundary_in_direction(&self, direction: &AbsoluteDirection) -> i32 {
        use AbsoluteDirection::*;
        match direction {
            North => self.y_max_boundary(),
            South => self.y_min_boundary(),
            East => self.x_max_boundary(),
            West => self.x_min_boundary(),
        }
    }

    /// Returns None if the requested coordinate is out of bounds.
    /// # Examples
    /// ```
    /// use tudi::{Bounds, BoundedMovingObject, RelativeDirection, Coordinate};
    /// let bounds = Bounds::new(-5, 10, -5, 10);
    /// let marker = BoundedMovingObject::try_from((&bounds, &Coordinate::default())).unwrap();
    /// // the marker is now at the origin facing north.
    /// assert_eq!(marker.coordinate_in_relative_direction(&RelativeDirection::Left), Some ( Coordinate {x:
    /// - 1, y: 0}));
    /// assert_eq!(marker.coordinate_in_relative_direction(&RelativeDirection::Right), Some ( Coordinate {x:
    /// 1, y: 0}));
    ///
    /// ```
    pub fn coordinate_in_relative_direction(&self, dir: &RelativeDirection) -> Option<Coordinate> {
        let candidate_coordinate = match dir {
            RelativeDirection::Left => {
                self.coordinate_in_direction(self.direction().incremented(), 1)
            }
            RelativeDirection::Right => {
                self.coordinate_in_direction(self.direction().decremented(), 1)
            }
        };

        if !self.is_within_bounds(&candidate_coordinate) {
            None
        } else {
            Some(candidate_coordinate)
        }
    }

    pub fn direction(&self) -> &AbsoluteDirection {
        &self.current_direction
    }

    pub fn set_current_direction(&mut self, direction: AbsoluteDirection) {
        self.current_direction = direction;
    }

    pub fn sum_of_current_coordinates(&self) -> i32 {
        self.current_pos.x.abs() + self.current_pos.y.abs()
    }

    pub fn set_current_x_to_x_min(&mut self) {
        self.current_pos.x = self.x_min_boundary();
    }

    pub fn set_current_x_to_x_max(&mut self) {
        self.current_pos.x = self.x_max_boundary();
    }

    pub fn set_current_y_to_y_min(&mut self) {
        self.current_pos.y = self.y_min_boundary();
    }

    pub fn set_current_y_to_y_max(&mut self) {
        self.current_pos.y = self.y_max_boundary();
    }

    pub fn coordinates_in_direction(&self, direction: AbsoluteDirection) -> Vec<Coordinate> {
        self.coordinates_in_direction_from(self.position(), direction)
    }

    fn set_boundary(&mut self, axis: Axis, minmax: MinMax, boundary: i32) -> Result<i32, String> {
        let [previous_min, previous_max, pos] = match axis {
            Axis::Y => [
                self.bounds.y_min_boundary(),
                self.bounds.y_max_boundary(),
                self.current_pos.y,
            ],
            Axis::X => [
                self.bounds.x_min_boundary(),
                self.bounds.x_max_boundary(),
                self.current_pos.x,
            ],
        };

        let [new_min, new_max] = match minmax {
            MinMax::Min => {
                if previous_max < boundary {
                    return Err("New min greater than previous min".to_string());
                } else if pos < boundary {
                    return Err("Current x-position smaller than new min!".to_string());
                } else {
                    [boundary, previous_max]
                }
            }

            MinMax::Max => {
                if previous_min > boundary {
                    return Err("New max smaller than previous min!".to_string());
                } else if pos > boundary {
                    return Err("Current y-positon greater than new max".to_string());
                } else {
                    [previous_min, boundary]
                }
            }
        };

        let new_bounds = match axis {
            Axis::Y => Bounds::new(
                self.bounds.x_min_boundary(),
                self.bounds.x_count(),
                new_min,
                (new_max - new_min) as usize,
            ),
            Axis::X => Bounds::new(
                new_min,
                (new_max - new_min) as usize,
                self.bounds.y_min_boundary(),
                self.bounds.y_count(),
            ),
        };
        self.bounds = new_bounds;
        Ok(boundary)
    }

    /// Create a BoundedMovingObject from a [Bounded].
    ///
    // This is a standalone rather than implementing From<Bounded> since this results in
    // conflicting blanket implementation in core (since BoundedMovingObject itself implements
    // Bounded)
    pub fn from_bounded<B: Bounded>(value: &B) -> Self {
        let x_min = value.x_min_boundary();
        let x_max = value.x_max_boundary();
        let y_min = value.y_min_boundary();
        let y_max = value.y_max_boundary();
        BoundedMovingObject::new(x_min, x_max, y_min, y_max)
    }
}

impl<B, C> TryFrom<(B, C)> for BoundedMovingObject
where
    B: Bounded,
    C: Positioned,
{
    type Error = OutOfBoundsError;

    fn try_from(value: (B, C)) -> Result<Self, Self::Error> {
        let mut result = Self::new(
            value.0.x_min_boundary(),
            value.0.x_max_boundary(),
            value.0.y_min_boundary(),
            value.0.y_max_boundary(),
        );

        if result.is_within_bounds(value.1.position()) {
            result.set_coordinate(value.1.position());
            Ok(result)
        } else {
            Err(OutOfBoundsError::new(value.1.position()))
        }
    }
}

impl From<Bounds> for BoundedMovingObject {
    fn from(value: Bounds) -> Self {
        let x_min = value.x_min_boundary();
        let x_max = value.x_max_boundary();
        let y_min = value.y_min_boundary();
        let y_max = value.y_max_boundary();
        BoundedMovingObject::new(x_min, x_max, y_min, y_max)
    }
}

impl Positioned for BoundedMovingObject {
    fn position(&self) -> &Coordinate {
        &self.current_pos
    }
}

impl Mover for BoundedMovingObject {
    fn set_coordinate<C: Positioned>(&mut self, coordinate: &C) {
        assert!(self.is_within_bounds(coordinate));
        self.current_pos = *coordinate.position();
    }
}

impl OriginCenteredness for BoundedMovingObject {
    type Distinguisher = MaybeOriginCentered;
}

impl MaybeOriginBounded for BoundedMovingObject {
    fn y_max(&self) -> i32 {
        self.bounds.y_max_boundary()
    }

    fn y_min(&self) -> i32 {
        self.bounds.y_min_boundary()
    }

    fn x_max(&self) -> i32 {
        self.bounds.x_max_boundary()
    }

    fn x_min(&self) -> i32 {
        self.bounds.x_min_boundary()
    }
}

enum Axis {
    X,
    Y,
}

enum MinMax {
    Min,
    Max,
}

impl DynamicallyBounded for BoundedMovingObject {
    fn set_y_max_boundary(&mut self, boundary: i32) -> Result<i32, String> {
        self.set_boundary(Axis::Y, MinMax::Max, boundary)
    }

    fn set_y_min_boundary(&mut self, boundary: i32) -> Result<i32, String> {
        self.set_boundary(Axis::Y, MinMax::Min, boundary)
    }

    fn set_x_max_boundary(&mut self, boundary: i32) -> Result<i32, String> {
        self.set_boundary(Axis::X, MinMax::Max, boundary)
    }

    fn set_x_min_boundary(&mut self, boundary: i32) -> Result<i32, String> {
        self.set_boundary(Axis::X, MinMax::Min, boundary)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Grid;

    fn create_at_origin() -> BoundedMovingObject {
        let x_min = 0;
        let x_max = 0;
        let y_min = 0;
        let y_max = 0;

        BoundedMovingObject::new(x_min, x_max, y_min, y_max)
    }

    fn check_boundary(input: &BoundedMovingObject, axis: Axis, minmax: MinMax, boundary: i32) {
        match (axis, minmax) {
            (Axis::Y, MinMax::Max) => assert_eq!(input.y_max_boundary(), boundary),
            (Axis::X, MinMax::Max) => assert_eq!(input.x_max_boundary(), boundary),
            (Axis::X, MinMax::Min) => assert_eq!(input.x_min_boundary(), boundary),
            (Axis::Y, MinMax::Min) => assert_eq!(input.y_min_boundary(), boundary),
        }
    }

    // check the count, also check if the geometric is correct if the count is correct.
    fn check_count_and_len(input: &BoundedMovingObject, axis: Axis, count: usize) {
        match axis {
            Axis::X => {
                assert_eq!(input.x_count(), count);
                if count > 0 {
                    assert_eq!(input.x_geometric_len(), count - 1)
                } else {
                    assert_eq!(input.x_geometric_len(), 0)
                }
            }

            Axis::Y => {
                assert_eq!(input.y_count(), count);
                if count > 0 {
                    assert_eq!(input.y_geometric_len(), count - 1)
                } else {
                    assert_eq!(input.y_geometric_len(), 0)
                }
            }
        }
    }

    #[test]
    pub fn create_from_bounds() {
        let bounds = Bounds::new(-5, 10, -5, 10);
        let pos = BoundedMovingObject::try_from((&bounds, &Coordinate::default())).unwrap();
        assert_eq!(pos.position(), &Coordinate::default());
    }

    #[test]
    pub fn simple_move_test() {
        let mut pos = BoundedMovingObject::new(-10, 10, -10, 10);
        let dir = RelativeDirection::Left;
        let mag = 2;
        pos.turn(dir);
        pos.move_in_current_direction_and_return_new_pos(mag);
        assert_eq!(pos.position(), &Coordinate { x: -2, y: 0 });
    }

    #[test]
    pub fn new_from_bounded() {
        let grid: Grid<BoundedMovingObject> = Grid::new(9, 9);
        let pos = BoundedMovingObject::try_from((&grid, &Coordinate::default())).unwrap();
        check_boundary(&pos, Axis::X, MinMax::Min, -4);
        check_boundary(&pos, Axis::X, MinMax::Max, 4);
        check_boundary(&pos, Axis::Y, MinMax::Min, -4);
        check_boundary(&pos, Axis::Y, MinMax::Max, 4);
    }

    #[test]
    pub fn simple_move_with_bound_test_y_neg() {
        let mut pos = BoundedMovingObject::new(-10, 10, -10, 10);
        pos.turn(RelativeDirection::Left);
        pos.turn(RelativeDirection::Left);
        pos.set_y_min_boundary(-5).unwrap();
        let magnitude = 2;
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, -2);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, -4);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, -5);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, -5);
    }

    #[test]
    pub fn simple_move_with_bound_test_x_neg() {
        let mut pos = BoundedMovingObject::new(-5, 100, -100, 100);
        pos.turn(RelativeDirection::Left);
        pos.set_x_min_boundary(-5).unwrap();
        let magnitude = 2;
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, -2);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, -4);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, -5);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, -5);
    }

    #[test]
    pub fn simple_move_with_bound_test_x_pos() {
        let mut pos = BoundedMovingObject::new(0, 5, -100, 100);
        pos.turn(RelativeDirection::Right);
        let magnitude = 2;
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, 2);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, 4);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, 5);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().x, 5);
    }

    #[test]
    pub fn simple_move_with_bound_test() {
        let mut pos = BoundedMovingObject::new(-100, 100, -100, 5);
        let magnitude = 2;
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, 2);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, 4);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, 5);
        pos.move_in_current_direction_and_return_new_pos(magnitude);
        assert_eq!(pos.position().y, 5);
    }

    #[test]
    pub fn neighbors_test() {
        let pos = BoundedMovingObject::new(0, 0, 0, 0);
        assert!(pos.get_bounded_neighbors().is_empty());
    }

    #[test]
    pub fn bounds_test_origin_only() {
        let x_min = 0;
        let x_max = 0;
        let y_min = 0;
        let y_max = 0;

        let pos = BoundedMovingObject::new(x_min, x_max, y_min, y_max);
        check_boundary(&pos, Axis::X, MinMax::Min, x_min);
        check_boundary(&pos, Axis::X, MinMax::Max, x_max);
        check_boundary(&pos, Axis::Y, MinMax::Min, y_min);
        check_boundary(&pos, Axis::Y, MinMax::Max, y_max);
    }

    #[test]
    pub fn bounds_test() {
        let pos = create_at_origin();
        check_boundary(&pos, Axis::X, MinMax::Min, 0);
        check_boundary(&pos, Axis::X, MinMax::Max, 0);
        check_boundary(&pos, Axis::Y, MinMax::Min, 0);
        check_boundary(&pos, Axis::Y, MinMax::Max, 0);
    }

    mod changing_bounds {
        use super::*;

        fn set(
            input: &mut BoundedMovingObject,
            axis: Axis,
            minmax: MinMax,
            boundary: i32,
        ) -> Result<i32, String> {
            match (axis, minmax) {
                (Axis::Y, MinMax::Max) => input.set_y_max_boundary(boundary),
                (Axis::X, MinMax::Max) => input.set_x_max_boundary(boundary),
                (Axis::X, MinMax::Min) => input.set_x_min_boundary(boundary),
                (Axis::Y, MinMax::Min) => input.set_y_min_boundary(boundary),
            }
        }

        #[test]
        fn valid_upwards_expansion() {
            let mut pos = create_at_origin();
            let _ = set(&mut pos, Axis::Y, MinMax::Max, 1);
            check_count_and_len(&pos, Axis::Y, 2);
            check_boundary(&pos, Axis::Y, MinMax::Max, 1);
        }

        #[test]
        fn valid_downwards_expansion() {
            let mut pos = create_at_origin();
            let _ = set(&mut pos, Axis::Y, MinMax::Min, -1);
            check_count_and_len(&pos, Axis::Y, 2);
            check_boundary(&pos, Axis::Y, MinMax::Min, -1);
        }

        #[test]
        fn valid_right_expansion() {
            let mut pos = create_at_origin();
            let _ = set(&mut pos, Axis::X, MinMax::Max, 1);
            check_count_and_len(&pos, Axis::X, 2);
            check_boundary(&pos, Axis::X, MinMax::Max, 1);
        }

        #[test]
        fn valid_left_expansion() {
            let mut pos = create_at_origin();
            let _ = set(&mut pos, Axis::X, MinMax::Min, -1);
            check_count_and_len(&pos, Axis::X, 2);
            check_boundary(&pos, Axis::X, MinMax::Min, -1);
        }
    }
}
