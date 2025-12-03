use crate::AbsoluteDirection;
use crate::Coordinate;
use crate::Positioned;
use crate::RelativeDirection;

#[derive(Default)]
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
        self.current_pos = self.coordinate_in_direction(*self.get_current_direction(), magnitude);
    }

    pub fn move_in_direction(&mut self, direction: &AbsoluteDirection, magnitude: u32) {
        self.current_pos = self.coordinate_in_direction(*direction, magnitude);
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

    #[test]
    pub fn simple_move_test() {
        let mut pos = MovingObject::default();
        let dir = RelativeDirection::Left;
        let mag = 2;
        pos.turn(dir);
        pos.move_in_current_direction(mag);

        assert_eq!(pos.position(), &Coordinate { x: -2, y: 0 });
    }

    #[test]
    pub fn simple_move_y_neg() {
        let mut pos = MovingObject::default();
        pos.turn(RelativeDirection::Left);
        pos.turn(RelativeDirection::Left);
        let magnitude = 2;
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().y, -2);
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().y, -4);
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().y, -6);
    }

    #[test]
    pub fn simple_move_with_x_neg() {
        let mut pos = MovingObject::default();
        pos.turn(RelativeDirection::Left);
        let magnitude = 2;
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().x, -2);
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().x, -4);
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().x, -6);
    }

    #[test]
    pub fn simple_move_test_x_pos() {
        let mut pos = MovingObject::default();
        pos.turn(RelativeDirection::Right);
        let magnitude = 2;
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().x, 2);
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().x, 4);
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().x, 6);
    }
    #[test]
    pub fn simple_move_y_with_test() {
        let mut pos = MovingObject::default();
        let magnitude = 2;
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().y, 2);
        pos.move_in_current_direction(magnitude);
        assert_eq!(pos.position().y, 4);
        pos.move_in_current_direction(magnitude);
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
