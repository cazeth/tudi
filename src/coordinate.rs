use crate::AbsoluteDirection;
use crate::Positioned;
use std::ops::Add;

/// A two-dimensional point.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Default, Hash)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn coordinate_in_direction(direction: &AbsoluteDirection, magnitude: usize) -> Self {
        let [x, y]: [i32; 2] = match direction {
            AbsoluteDirection::North => [0, magnitude as i32],
            AbsoluteDirection::South => [0, -(magnitude as i32)],
            AbsoluteDirection::East => [magnitude as i32, 0],
            AbsoluteDirection::West => [-(magnitude as i32), 0],
        };
        Self { x, y }
    }

    pub fn move_in_direction(&mut self, direction: &AbsoluteDirection, magnitude: usize) {
        use AbsoluteDirection::*;

        match direction {
            North => self.y += magnitude as i32,
            East => self.x += magnitude as i32,
            West => self.x -= magnitude as i32,
            South => self.y -= magnitude as i32,
        }
    }

    /// Checks if the coordinate is above a row. If the coordinate is on the row the function returns true.
    pub fn is_above_row(&self, row: i32) -> bool {
        self.y_coordinate() >= row
    }

    /// Checks if the coordinate is below a row. If the coordinate is on the row the function returns true.
    pub fn is_below_row(&self, row: i32) -> bool {
        self.y_coordinate() <= row
    }
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Positioned for Coordinate {
    fn position(&self) -> &Coordinate {
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    pub fn simple_manhattan_distance() {
        assert_eq!(Coordinate { x: 5, y: 5 }.manhattan_distance_to_origin(), 10);
        assert_eq!(
            Coordinate { x: 5, y: -5 }.manhattan_distance_to_origin(),
            10
        );
        assert_eq!(
            Coordinate { x: -5, y: 5 }.manhattan_distance_to_origin(),
            10
        );
        assert_eq!(
            Coordinate { x: -5, y: -5 }.manhattan_distance_to_origin(),
            10
        );
    }

    #[test]
    pub fn test_move() {
        let mut coordinate = Coordinate { x: 10, y: 10 };
        coordinate.move_in_direction(&AbsoluteDirection::North, 1);
        assert_eq!(coordinate, Coordinate { x: 10, y: 11 });
        coordinate.move_in_direction(&AbsoluteDirection::South, 1);
        assert_eq!(coordinate, Coordinate { x: 10, y: 10 });
        coordinate.move_in_direction(&AbsoluteDirection::South, 10);
        assert_eq!(coordinate, Coordinate { x: 10, y: 0 });
        coordinate.move_in_direction(&AbsoluteDirection::West, 10);
        assert_eq!(coordinate, Coordinate { x: 0, y: 0 });
    }

    #[test]
    pub fn add_and_manhattan_distance() {
        assert_eq!(
            (Coordinate { x: 5, y: 5 } + Coordinate { x: 1, y: 0 }).manhattan_distance_to_origin(),
            11
        );
    }

    #[test]
    pub fn manhattan_neighbors() {
        let neighbors = Coordinate::default().manhattan_neighbors();
        assert_eq!(neighbors.len(), 4);
        for direction in [
            AbsoluteDirection::North,
            AbsoluteDirection::South,
            AbsoluteDirection::West,
            AbsoluteDirection::East,
        ] {
            assert!(
                neighbors.contains(&Coordinate::default().coordinate_in_direction(direction, 1),)
            );
        }
    }

    #[test]
    pub fn manhattan_neighbors_distance() {
        for x in -100..100 {
            for y in -100..100 {
                let main_coordinate = Coordinate { x, y };
                let neighbors = main_coordinate.manhattan_neighbors();
                assert!(neighbors.iter().all(|x| {
                    x.manhattan_distance_to_origin()
                        .abs_diff(main_coordinate.manhattan_distance_to_origin())
                        == 1
                }))
            }
        }
    }

    #[test]
    pub fn should_be_above_row() {
        let c = Coordinate::default();
        assert!(c.is_above_row(-2));
        assert!(c.is_above_row(0));
        assert!(!c.is_above_row(1));
    }

    #[test]
    pub fn should_be_below_row() {
        let c = Coordinate::default();
        assert!(!c.is_below_row(-2));
        assert!(c.is_below_row(0));
        assert!(c.is_below_row(1));
    }
}
