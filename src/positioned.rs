#![allow(clippy::enum_glob_use)]
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

    /// The Manhattan distance to another [`Positioned`].
    ///
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

    /// The immediately surrounding coordinates to self, not including
    /// diagonals.
    ///
    /// See also [`Positioned::euclid_neighbors()`]
    ///
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

    /// The immediately surrounding coordinates to self, including diagonals.
    ///
    /// See also [`Positioned::manhattan_neighbors()`]
    ///
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

    /// Subtract a coordinate from self.
    ///
    /// Note that this method returns the signed difference rather than the absolute x/y distances.
    fn difference(&self, other: &Self) -> Coordinate
    where
        Self: Sized,
    {
        let x = self.x_coordinate() - other.x_coordinate();
        let y = self.y_coordinate() - other.y_coordinate();
        Coordinate { x, y }
    }

    /// The [`AbsoluteDirection`] from self to another coordinate.
    ///
    /// If the direction is an exact absolute direction (for instance, straight north), the method returns `(Some(AbsoluteDirection), None)`. If the direction is a combination of directions, such as northwest, the method returns `(Some(North), Some(West))`. If the target is at the same position as the source, the method returns `(None, None)`.
    fn direction_toward(
        &self,
        target: &Coordinate,
    ) -> (Option<AbsoluteDirection>, Option<AbsoluteDirection>) {
        // Handles when there is an exact direction to target (exactly north, south, east, west).
        if self.position() == target.position() {
            return (None, None);
        } else if target.x_coordinate() == self.x_coordinate() {
            if target.y_coordinate() > self.y_coordinate() {
                return (Some(AbsoluteDirection::North), None);
            } else {
                return (Some(AbsoluteDirection::South), None);
            }
        } else if target.y_coordinate() == self.y_coordinate() {
            if target.x_coordinate() > self.x_coordinate() {
                return (Some(AbsoluteDirection::East), None);
            } else {
                return (Some(AbsoluteDirection::West), None);
            }
        };

        let first_direction = if target.y_coordinate() > self.y_coordinate() {
            Some(AbsoluteDirection::North)
        } else {
            Some(AbsoluteDirection::South)
        };
        let second_direction = if target.x_coordinate() > self.x_coordinate() {
            Some(AbsoluteDirection::East)
        } else {
            Some(AbsoluteDirection::West)
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

    /// Max allowed magnitude is 2^31. Numbers larger than this yield undefined behavior.
    /// If the current coordinate + magnitude is larger than 2^32, that will also trigger
    /// undefined behavior.
    fn coordinate_in_direction(&self, direction: AbsoluteDirection, magnitude: u32) -> Coordinate {
        use AbsoluteDirection::*;
        match direction {
            North => Coordinate {
                x: self.x_coordinate(),
                y: self.y_coordinate() + i32::try_from(magnitude).unwrap_or(i32::MAX),
            },

            South => Coordinate {
                x: self.x_coordinate(),
                y: self.y_coordinate() - i32::try_from(magnitude).unwrap_or(i32::MAX),
            },

            East => Coordinate {
                x: self.x_coordinate() + i32::try_from(magnitude).unwrap_or(i32::MAX),
                y: self.y_coordinate(),
            },

            West => Coordinate {
                x: self.x_coordinate() - i32::try_from(magnitude).unwrap_or(i32::MAX),
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

#[cfg(test)]
pub mod test {
    macro_rules! check_direction {
        (from $source:ident to $target:expr => none) => {{
            let direction = $source.direction_toward(&$target);
            assert_eq!(direction, (None, None));
        }};
        (from $source:ident to $target:expr => $first:expr, $second:expr) => {{
            #[allow(unused_imports)]
            use $crate::AbsoluteDirection::*;
            let direction = $source.direction_toward(&$target);
            assert_eq!(direction, (Some($first), Some($second)));
        }};
        (from $source:ident to $target:expr => $first:expr) => {{
            #[allow(unused_imports)]
            use $crate::AbsoluteDirection::*;
            let direction = $source.direction_toward(&$target);
            assert_eq!(direction, (Some($first), None));
        }};
    }

    pub(crate) use check_direction;
}
