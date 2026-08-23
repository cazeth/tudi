use crate::AbsoluteDirection;
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
#[derive(Clone, Debug, Copy)]
pub struct BoundedMovingObject {
    current_pos: Coordinate,
    current_direction: AbsoluteDirection,
    bounds: Bounds,
}

impl BoundedMovingObject {
    /// A new BoundedMovingObject.
    ///
    /// If the origin is in bounds, the marker is set to the
    /// origin. If not, it is set to the northwest corner.
    ///
    /// For each axis, the function checks which of the provided boundaries for an axis is the smallest and sets the minimum boundary to it and the other one to the maximum boundary.
    fn new(
        first_x_boundary: i32,
        second_x_boundary: i32,
        first_y_boundary: i32,
        second_y_boundary: i32,
    ) -> Self {
        let bounds = Bounds::from_boundaries(
            first_x_boundary,
            second_x_boundary,
            first_y_boundary,
            second_y_boundary,
        );

        let current_pos = if bounds.is_within_bounds(&Coordinate::default()) {
            Coordinate::default()
        } else {
            bounds.northwest_corner()
        };

        Self {
            current_pos,
            bounds,
            current_direction: AbsoluteDirection::North,
        }
    }

    pub fn turn_toward<C: Positioned>(&mut self, target: &C) -> Result<&AbsoluteDirection, String> {
        match self.direction_toward(target.position()) {
            (None, _) => Err("Target and source have the same position".to_string()),
            (Some(_), Some(_)) => Err("No clean turn".to_string()),
            (Some(first), None) => {
                self.current_direction = first;
                Ok(&self.current_direction)
            }
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
    /// it possible to keep track of what happened.
    pub fn move_in_current_direction_and_return_new_pos(&mut self, magnitude: u32) -> Coordinate {
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
                    return Err("Current y-position greater than new max".to_string());
                } else {
                    [previous_min, boundary]
                }
            }
        };

        let new_bounds = match axis {
            Axis::Y => Bounds::from_boundaries(
                self.bounds.x_min_boundary(),
                self.bounds.x_max_boundary(),
                new_min,
                new_max,
            ),
            Axis::X => Bounds::from_boundaries(
                new_min,
                new_max,
                self.bounds.y_min_boundary(),
                self.bounds.y_max_boundary(),
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

        if let Some((first_direction, second_direction)) = result.out_of_bounds_directions(&value.1)
        {
            Err(OutOfBoundsError::new(
                value.1.position(),
                first_direction,
                second_direction,
            ))
        } else {
            result.set_coordinate(value.1.position());
            Ok(result)
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
    use crate::bounded::test::check_x_count;
    use crate::bounded::test::check_x_max;
    use crate::bounded::test::check_x_min;
    use crate::bounded::test::check_y_count;
    use crate::bounded::test::check_y_max;
    use crate::bounded::test::check_y_min;

    fn create_at_origin() -> BoundedMovingObject {
        let x_min = 0;
        let x_max = 0;
        let y_min = 0;
        let y_max = 0;

        BoundedMovingObject::new(x_min, x_max, y_min, y_max)
    }

    #[test]
    fn outside_origin_starts_at_northwest_corner() {
        let object = BoundedMovingObject::new(5, 10, 5, 10);

        assert_eq!(object.position(), &Coordinate { x: 5, y: 10 });
    }

    #[test]
    pub fn create_from_bounds() {
        #[expect(deprecated)]
        let bounds = Bounds::new(-5, 10, -5, 10);
        let pos = BoundedMovingObject::try_from((&bounds, &Coordinate::default())).unwrap();
        assert_eq!(pos.position(), &Coordinate::default());
    }

    /// Run a full test of a [`BoundedMovingObject`]'s movement.
    ///
    /// Provide a starting bounds, (optionally) a starting position, a chain of instructions to
    /// execute, and an expected final position.
    ///
    /// syntax is
    /// [<northwest-corner>, <southeast-corner>], <execution-chain> => <expected-position>
    ///
    /// where positions are (x,y).
    ///
    /// For instance:
    ///
    /// check_movement!([(-5,5), (5,-5)], l 3 => (-3,0))
    ///
    /// asserts that an object walking three steps to west from the origin in a ten-by-ten bounded region ends up at (-3,0).
    ///
    /// You can also provide the starting point:
    ///
    /// check_movement!([(-5,5), (5,-5)], start (5,0), l 5 => (0,0))
    ///
    /// starts at (5,0) and walks back to the origin.
    ///
    macro_rules! check_movement {
        ([$nw:tt, $se:tt], start $start:tt, $($rest:tt)*) => {{
            let mut object = BoundedMovingObject::new($nw.0, $se.0, $nw.1, $se.1);

            let starting_coordinate = Coordinate {
                x: $start.0,
                y: $start.1,
            };

            object.set_coordinate(&starting_coordinate);
            check_movement!(@execute object; $($rest)*)
        }};

        ([$nw:tt, $se:tt], $($rest:tt)*) => {{
            let mut object = BoundedMovingObject::new($nw.0, $se.0, $nw.1, $se.1);
            check_movement!(@execute object; $($rest)*)
        }};

        (@execute $object:ident; => $final:tt) => {{
            let final_coordinate = $crate::Coordinate {
                x: $final.0,
                y: $final.1,
            };

            assert_eq!($object.position(), &final_coordinate);
            $object
        }};

        (@execute $object:ident; $instruction:tt $($rest:tt)*) => {{
            execute!(@one $object $instruction);
            check_movement!(@execute $object; $($rest)*)
        }};
    }

    /// a simple dsl for concisely expressing the movement of an object. See the branches for the
    /// specification.
    ///
    /// For instance:
    /// execute!(object, l 5 l 10) turns left and walks five steps, then turns left and walks ten
    /// steps.
    macro_rules! execute {
        (@one $object:ident l) => {$object.turn(crate::RelativeDirection::Left);};
        (@one $object:ident r) => {$object.turn(crate::RelativeDirection::Right);};
        (@one $object:ident n) => {$object.set_current_direction(crate::AbsoluteDirection::North);};
        (@one $object:ident s) => {$object.set_current_direction(crate::AbsoluteDirection::South);};
        (@one $object:ident e) => {$object.set_current_direction(crate::AbsoluteDirection::East);};
        (@one $object:ident w) => {$object.set_current_direction(crate::AbsoluteDirection::West);};
        (@one $object:ident 1) => {$object.move_in_current_direction_and_return_new_pos(1);};
        (@one $object:ident 2) => {$object.move_in_current_direction_and_return_new_pos(2);};
        (@one $object:ident 3) => {$object.move_in_current_direction_and_return_new_pos(3);};
        (@one $object:ident 4) => {$object.move_in_current_direction_and_return_new_pos(4);};
        (@one $object:ident 5) => {$object.move_in_current_direction_and_return_new_pos(5);};
        (@one $object:ident 6) => {$object.move_in_current_direction_and_return_new_pos(6);};
        (@one $object:ident 7) => {$object.move_in_current_direction_and_return_new_pos(7);};
        (@one $object:ident 8) => {$object.move_in_current_direction_and_return_new_pos(8);};
        (@one $object:ident 9) => {$object.move_in_current_direction_and_return_new_pos(9);};
        ($object:ident, $($instruction:tt)*) => {{
            $( execute!(@one $object $instruction);)*
        }};
    }

    #[test]
    pub fn simple_move_test() {
        check_movement!([(-10,10), (10,-10)], l 2 => (-2,0));
        check_movement!([(0,0), (0,0)], l 2 => (0,0));
        check_movement!([(-3,3), (3,-3)], 3 l 3 l 6 l 6 l 6 l 3 l 3 l => (0,0));
        check_movement!([(-3,3), (3,-3)], 3 l l 3 => (0,0));
        check_movement!([(-5,100), (5,-100)], r 2 2 2 2 => (5,0));
        check_movement!([(-5,5), (5,-5)], start (5,0), l 5 => (0,0));
        check_movement!([(-100,5), (100,-100)], 2 2 2 2  => (0,5));
        check_movement!([(5,10), (10,5)], e 5 s 5 => (10,5));
        check_movement!([(-10,10), (10,-10)], n 5 e 5 s 5 w 5 => (0,0));
    }

    #[test]
    pub fn new_from_bounded() {
        #[expect(deprecated)]
        let bounds = Bounds::new(-4, 8, -4, 8);
        let pos = BoundedMovingObject::try_from((&bounds, &Coordinate::default())).unwrap();
        check_x_min(&pos, -4);
        check_x_max(&pos, 4);
        check_y_min(&pos, -4);
        check_y_max(&pos, 4);
    }

    #[test]
    pub fn simple_move_with_bound_test_y_neg() {
        let mut pos = BoundedMovingObject::new(-10, 10, -10, 10);
        execute!(pos, l l);
        pos.set_y_min_boundary(-5).unwrap();
        execute!(pos, 2 2 2 2);
        assert_eq!(pos.position().y, -5);
    }

    #[test]
    pub fn simple_move_with_bound_test_x_neg() {
        let mut pos = BoundedMovingObject::new(-5, 100, -100, 100);
        pos.set_x_min_boundary(-5).unwrap();
        execute!(pos, l 2 2 2 2);
        assert_eq!(pos.position().x, -5);
    }

    #[test]
    pub fn neighbors_test() {
        let pos = BoundedMovingObject::new(0, 0, 0, 0);
        assert!(pos.bounded_neighbors().is_empty());
    }

    #[test]
    pub fn bounds_test_origin_only() {
        let x_min = 0;
        let x_max = 0;
        let y_min = 0;
        let y_max = 0;

        let pos = BoundedMovingObject::new(x_min, x_max, y_min, y_max);
        check_x_min(&pos, x_min);
        check_x_max(&pos, x_max);
        check_y_min(&pos, y_min);
        check_y_max(&pos, y_max);
    }

    #[test]
    pub fn bounds_test() {
        let pos = create_at_origin();
        check_x_min(&pos, 0);
        check_x_max(&pos, 0);
        check_y_min(&pos, 0);
        check_y_max(&pos, 0);
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
            check_y_count(&pos, 2);
            check_y_max(&pos, 1);
        }

        #[test]
        fn valid_downwards_expansion() {
            let mut pos = create_at_origin();
            let _ = set(&mut pos, Axis::Y, MinMax::Min, -1);
            check_y_count(&pos, 2);
            check_y_min(&pos, -1);
        }

        #[test]
        fn valid_right_expansion() {
            let mut pos = create_at_origin();
            let _ = set(&mut pos, Axis::X, MinMax::Max, 1);
            check_x_count(&pos, 2);
            check_x_max(&pos, 1);
        }

        #[test]
        fn valid_left_expansion() {
            let mut pos = create_at_origin();
            let _ = set(&mut pos, Axis::X, MinMax::Min, -1);
            check_x_count(&pos, 2);
            check_x_min(&pos, -1);
        }
    }

    mod direction_toward {

        use super::*;
        use crate::positioned::test::check_direction;

        #[test]
        fn test_basic_directions() {
            let source = create_at_origin();

            check_direction![from source to Coordinate{x:0,y:1} => North];
            check_direction![from source to Coordinate{x:1,y:1} => North, East];
            check_direction![from source to Coordinate{x:1,y:0} => East];
            check_direction![from source to Coordinate{x:1,y:-1} => South, East];
            check_direction![from source to Coordinate{x:0,y:-1} => South];
            check_direction![from source to Coordinate{x:-1,y:-1} => South, West];
            check_direction![from source to Coordinate{x:-1,y:0} => West];
            check_direction![from source to Coordinate{x:-1,y:1} => North, West];
            check_direction![from source to Coordinate::default() => none];
        }
    }
}
