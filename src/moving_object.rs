use crate::AbsoluteDirection;
use crate::Coordinate;
use crate::Positioned;
use crate::RelativeDirection;

#[derive(Default, Debug)]
pub struct MovingObject {
    current_pos: Coordinate,
    current_direction: AbsoluteDirection,
}

impl MovingObject {
    pub fn new(current_pos: Coordinate) -> Self {
        Self {
            current_pos,
            current_direction: AbsoluteDirection::North,
        }
    }

    pub fn turn(&mut self, dir: RelativeDirection) {
        match dir {
            RelativeDirection::Left => {
                self.set_current_direction(self.get_current_direction().increment());
            }
            RelativeDirection::Right => {
                self.set_current_direction(self.get_current_direction().decrement());
            }
        }
    }

    pub fn move_in_current_direction(&mut self, magnitude: u32) {
        self.current_pos = self.coordinate_in_direction(
            *self.get_current_direction(),
            magnitude
                .try_into()
                .expect("Magnitude is greater than AxisLength::MAX"),
        );
    }

    pub fn move_in_direction(&mut self, direction: &AbsoluteDirection, magnitude: u32) {
        self.current_pos = self.coordinate_in_direction(*direction, magnitude.try_into().unwrap());
    }

    pub fn get_current_direction(&self) -> &AbsoluteDirection {
        &self.current_direction
    }

    pub fn set_current_direction(&mut self, direction: AbsoluteDirection) {
        self.current_direction = direction;
    }

    pub fn get_sum_of_current_coordinates(&self) -> i32 {
        self.current_pos.x.abs() + self.current_pos.y.abs()
    }
}

impl Positioned for MovingObject {
    fn position(&self) -> &Coordinate {
        &self.current_pos
    }
}

#[cfg(test)]
mod tests {

    use super::*;

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
        (@one $object:ident 1) => {$object.move_in_current_direction(1);};
        (@one $object:ident 2) => {$object.move_in_current_direction(2);};
        (@one $object:ident 3) => {$object.move_in_current_direction(3);};
        (@one $object:ident 4) => {$object.move_in_current_direction(4);};
        (@one $object:ident 5) => {$object.move_in_current_direction(5);};
        (@one $object:ident 6) => {$object.move_in_current_direction(6);};
        (@one $object:ident 7) => {$object.move_in_current_direction(7);};
        (@one $object:ident 8) => {$object.move_in_current_direction(8);};
        (@one $object:ident 9) => {$object.move_in_current_direction(9);};
        ($object:ident, $($instruction:tt)*) => {{
            $( execute!(@one $object $instruction);)*
        }};
    }

    #[test]
    pub fn simple_move_test() {
        let mut pos = MovingObject::default();
        execute!(pos, l 2);
        assert_eq!(pos.position(), &Coordinate { x: -2, y: 0 });
    }

    #[test]
    pub fn simple_move_y_neg() {
        let mut pos = MovingObject::default();
        execute!(pos, l l 2);
        assert_eq!(pos.position().y, -2);
        execute!(pos, 2);
        assert_eq!(pos.position().y, -4);
        execute!(pos, 2);
        assert_eq!(pos.position().y, -6);
    }

    #[test]
    pub fn simple_move_with_x_neg() {
        let mut pos = MovingObject::default();
        execute!(pos, l 2);
        assert_eq!(pos.position().x, -2);
        execute!(pos, 2);
        assert_eq!(pos.position().x, -4);
        execute!(pos, 2);
        assert_eq!(pos.position().x, -6);
    }

    #[test]
    pub fn simple_move_test_x_pos() {
        let mut pos = MovingObject::default();
        execute!(pos, r 2);
        assert_eq!(pos.position().x, 2);
        execute!(pos, 2);
        assert_eq!(pos.position().x, 4);
        execute!(pos, 2);
        assert_eq!(pos.position().x, 6);
    }
    #[test]
    pub fn simple_move_y_with_test() {
        let mut pos = MovingObject::default();
        execute!(pos, 2);
        assert_eq!(pos.position().y, 2);
        execute!(pos, 2);
        assert_eq!(pos.position().y, 4);
        execute!(pos, 2);
        assert_eq!(pos.position().y, 6);
    }

    pub fn neighbor_test(x_diff: u32, y_diff: u32) {
        let mut pos = MovingObject::default();
        pos.move_in_direction(&AbsoluteDirection::East, x_diff);
        pos.move_in_direction(&AbsoluteDirection::North, y_diff);
        let neighbors = pos.euclid_neighbors();
        assert_eq!(neighbors.len(), 8);

        assert!(neighbors.contains(&Coordinate {
            x: 1 + x_diff as i32,
            y: y_diff as i32
        }));
        assert!(neighbors.contains(&Coordinate {
            x: 1 + x_diff as i32,
            y: 1 + y_diff as i32
        }));
        assert!(neighbors.contains(&Coordinate {
            x: x_diff as i32,
            y: 1 + y_diff as i32
        }));
        assert!(neighbors.contains(&Coordinate {
            x: -1 + x_diff as i32,
            y: 1 + y_diff as i32
        }));

        assert!(neighbors.contains(&Coordinate {
            x: -1 + x_diff as i32,
            y: y_diff as i32
        }));
        assert!(neighbors.contains(&Coordinate {
            x: -1 + x_diff as i32,
            y: -1 + y_diff as i32
        }));
        assert!(neighbors.contains(&Coordinate {
            x: x_diff as i32,
            y: -1 + y_diff as i32
        }));
        assert!(neighbors.contains(&Coordinate {
            x: 1 + x_diff as i32,
            y: -1 + y_diff as i32
        }));
    }

    #[test]
    pub fn neighbor_tests() {
        for y_diff in 0..100 {
            for x_diff in 0..100 {
                neighbor_test(x_diff, y_diff);
            }
        }
    }
}
