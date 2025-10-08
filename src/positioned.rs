use crate::AbsoluteDirection;
use crate::Coordinate;

pub trait Positioned {
    fn position(&self) -> &Coordinate;

    fn x_coordinate(&self) -> i32 {
        self.position().x
    }

    fn y_coordinate(&self) -> i32 {
        self.position().y
    }

    fn manhattan_distance_to_origin(&self) -> usize {
        self.x_coordinate().unsigned_abs() as usize + self.y_coordinate().unsigned_abs() as usize
    }

    /// Returns the manhattan distance to another OnGrid Object.
    /// # Examples
    /// ```
    /// use tudi::Coordinate;
    /// use tudi::Positioned;
    /// let coord_1 = Coordinate{ x : -1, y : 3};
    /// let coord_2 = Coordinate{ x : 2, y : -7};
    /// assert_eq!(coord_1.manhattan_distance_to(&coord_2), 13 );
    /// ```
    fn manhattan_distance_to<C: Positioned>(&self, cord: &C) -> usize
    where
        Self: Sized,
    {
        (self.x_coordinate() - cord.x_coordinate()).unsigned_abs() as usize
            + (self.y_coordinate() - cord.y_coordinate()).unsigned_abs() as usize
    }

    /// Return a vec of the immediately surrounding coordinates to the current coordinate, not considering
    /// diagonals.
    /// # Examples
    /// ```
    /// use tudi::MovingObject;
    /// use tudi::Coordinate;
    /// use tudi::Positioned;
    ///
    /// let pos = MovingObject::default();
    /// // default is origin
    ///
    /// assert!(pos.manhattan_neighbors().contains(&Coordinate {x: 1, y : 0}));
    /// assert!(!pos.manhattan_neighbors().contains(&Coordinate {x: 1, y : 1}));
    /// ```
    fn manhattan_neighbors(&self) -> Vec<Coordinate> {
        let mut result: Vec<Coordinate> = Vec::new();
        use AbsoluteDirection::*;
        for direction in [North, East, South, West] {
            result.push(self.coordinate_in_direction(direction, 1))
        }

        result
    }

    /// Return a vec of the immediately surrounding coordinates to the current coordinate, not considering
    /// diagonals.
    /// # Examples
    /// ```
    /// use tudi::MovingObject;
    /// use tudi::Coordinate;
    /// use tudi::Positioned;
    ///
    /// let pos = MovingObject::default();
    /// // default is origin
    ///
    /// assert!(pos.euclid_neighbors().contains(&Coordinate {x: 1, y : 0}));
    /// assert!(pos.euclid_neighbors().contains(&Coordinate {x: 1, y : 1}));
    /// ```
    fn euclid_neighbors(&self) -> Vec<Coordinate> {
        let mut result: Vec<Coordinate> = Vec::new();
        use AbsoluteDirection::*;
        for direction in [North, East, South, West] {
            result.push(self.coordinate_in_direction(direction, 1))
        }

        for first_direction in [North, South] {
            for second_direction in [East, West] {
                result.push(
                    self.coordinate_in_direction(first_direction, 1)
                        .coordinate_in_direction(second_direction, 1),
                )
            }
        }

        result
    }

    /// subtract the coordinates.
    /// Note that this method returns the signed difference rather than the absolute x/y distances.
    fn difference(&self, other: &Self) -> Coordinate
    where
        Self: Sized,
    {
        let x = self.x_coordinate() - other.x_coordinate();
        let y = self.y_coordinate() - other.y_coordinate();
        Coordinate { x, y }
    }

    /// returns the absolute directions from self to another coordinate. If the direction in an
    /// exact direction (for instance , straight north) it returns that direction twice.
    fn direction_toward(&self, target: &Coordinate) -> (AbsoluteDirection, AbsoluteDirection) {
        //handles when there is an exact direction to target (eactly north, south, east, west)
        if self.position() == target.position() {
            panic!();
        } else if target.x_coordinate() == self.x_coordinate() {
            if target.y_coordinate() > self.y_coordinate() {
                return (AbsoluteDirection::North, AbsoluteDirection::North);
            } else {
                return (AbsoluteDirection::South, AbsoluteDirection::South);
            }
        } else if target.y_coordinate() == self.y_coordinate() {
            if target.x_coordinate() > self.x_coordinate() {
                return (AbsoluteDirection::East, AbsoluteDirection::East);
            } else {
                return (AbsoluteDirection::West, AbsoluteDirection::West);
            }
        };

        // handles when there is two direction, northeast, southwest ....
        let first_direction = if target.y_coordinate() > self.y_coordinate() {
            AbsoluteDirection::North
        } else {
            AbsoluteDirection::South
        };
        let second_direction = if target.x_coordinate() > self.x_coordinate() {
            AbsoluteDirection::East
        } else {
            AbsoluteDirection::West
        };
        (first_direction, second_direction)
    }

    fn on_opposite_sides_of_row(&self, cord: &Self, row: &i32) -> bool
    where
        Self: Sized,
    {
        (&self.y_coordinate() > row && row > &cord.y_coordinate())
            || (&cord.y_coordinate() > row && row > &self.y_coordinate())
    }

    fn on_opposite_sides_of_column(&self, cord: &Self, row: &i32) -> bool
    where
        Self: Sized,
    {
        (&self.x_coordinate() > row && row > &cord.x_coordinate())
            || (&cord.x_coordinate() > row && row > &self.x_coordinate())
    }

    fn coordinate_in_direction(
        &self,
        direction: AbsoluteDirection,
        magnitude: usize,
    ) -> Coordinate {
        use AbsoluteDirection::*;
        match direction {
            North => Coordinate {
                x: self.x_coordinate(),
                y: self.y_coordinate() + magnitude as i32,
            },

            South => Coordinate {
                x: self.x_coordinate(),
                y: self.y_coordinate() - magnitude as i32,
            },

            East => Coordinate {
                x: self.x_coordinate() + magnitude as i32,
                y: self.y_coordinate(),
            },

            West => Coordinate {
                x: self.x_coordinate() - magnitude as i32,
                y: self.y_coordinate(),
            },
        }
    }
}

impl<T: Positioned> Positioned for &T {
    fn position(&self) -> &Coordinate {
        T::position(self)
    }
}
