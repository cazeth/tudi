use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Default, Copy)]
pub enum AbsoluteDirection {
    East,
    #[default]
    North,
    West,
    South,
}

impl fmt::Display for AbsoluteDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction = match self {
            Self::East => "East",
            Self::North => "North",
            Self::West => "West",
            Self::South => "South",
        };

        formatter.write_str(direction)
    }
}

impl AbsoluteDirection {
    pub fn turn(self, turning_direction: &RelativeDirection) -> Self {
        match turning_direction {
            RelativeDirection::Left => self.increment(),
            RelativeDirection::Right => self.decrement(),
        }
    }

    pub fn turned(&self, turning_direction: &RelativeDirection) -> Self {
        match turning_direction {
            RelativeDirection::Left => self.incremented(),
            RelativeDirection::Right => self.decremented(),
        }
    }

    /// Turn in counter-clockwise direction.
    pub fn increment(self) -> Self {
        use AbsoluteDirection::*;
        match self {
            East => North,
            North => West,
            West => South,
            South => East,
        }
    }

    /// Turn in clockwise direction.
    pub fn decrement(self) -> Self {
        use AbsoluteDirection::*;
        match self {
            East => South,
            North => East,
            West => North,
            South => West,
        }
    }

    pub fn incremented(&self) -> Self {
        use AbsoluteDirection::*;
        match self {
            East => North,
            North => West,
            West => South,
            South => East,
        }
    }

    pub fn decremented(&self) -> Self {
        use AbsoluteDirection::*;
        match self {
            East => South,
            North => East,
            West => North,
            South => West,
        }
    }

    /// Returns the opposite direction: North -> South, East -> West and vice versa.
    /// ```
    /// use tudi::AbsoluteDirection;
    /// assert_eq!(AbsoluteDirection::North.inverse(), AbsoluteDirection::South);
    /// ```
    pub fn inverse(&self) -> Self {
        use AbsoluteDirection::*;
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }

    /// Returns the relative direction between two directions when possible, otherwise return None.
    /// ```
    /// # use tudi::AbsoluteDirection;
    /// # use tudi::RelativeDirection::*;
    /// # use tudi::AbsoluteDirection::*;
    /// assert_eq!(AbsoluteDirection::to_relative_direction(&North, &East), Some(Right));
    /// assert_eq!(AbsoluteDirection::to_relative_direction(&East, &North), Some(Left));
    /// assert_eq!(AbsoluteDirection::to_relative_direction(&North, &South), None );
    /// assert_eq!(AbsoluteDirection::to_relative_direction(&North, &West), Some(Left) );
    /// ```
    pub fn to_relative_direction(
        first_direction: &Self,
        second_direction: &Self,
    ) -> Option<RelativeDirection> {
        let first_direction_score = Self::get_direction_score(first_direction);
        let second_direction_score = Self::get_direction_score(second_direction);
        if (first_direction_score + 1) % 4 == second_direction_score {
            Some(RelativeDirection::Right)
        } else if (first_direction_score + 3) % 4 == second_direction_score {
            Some(RelativeDirection::Left)
        } else {
            None
        }
    }

    fn get_direction_score(direction: &Self) -> usize {
        use AbsoluteDirection::*;

        match direction {
            North => 0,
            East => 1,
            South => 2,
            West => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum VerticalDirection {
    #[default]
    North,
    South,
}

impl fmt::Display for VerticalDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction = match self {
            Self::North => "North",
            Self::South => "South",
        };

        formatter.write_str(direction)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum HorizontalDirection {
    #[default]
    East,
    West,
}

impl fmt::Display for HorizontalDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction = match self {
            Self::East => "East",
            Self::West => "West",
        };

        formatter.write_str(direction)
    }
}

impl From<VerticalDirection> for AbsoluteDirection {
    fn from(value: VerticalDirection) -> Self {
        match value {
            VerticalDirection::North => AbsoluteDirection::North,
            VerticalDirection::South => AbsoluteDirection::South,
        }
    }
}

impl From<HorizontalDirection> for AbsoluteDirection {
    fn from(value: HorizontalDirection) -> Self {
        match value {
            HorizontalDirection::East => AbsoluteDirection::East,
            HorizontalDirection::West => AbsoluteDirection::West,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RelativeDirection {
    Left,
    Right,
}

impl fmt::Display for RelativeDirection {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let direction = match self {
            Self::Left => "Left",
            Self::Right => "Right",
        };

        formatter.write_str(direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(AbsoluteDirection::North.to_string(), "North");
        assert_eq!(AbsoluteDirection::South.to_string(), "South");
        assert_eq!(AbsoluteDirection::East.to_string(), "East");
        assert_eq!(AbsoluteDirection::West.to_string(), "West");
    }

    #[test]
    fn display_horizontal_direction() {
        assert_eq!(HorizontalDirection::East.to_string(), "East");
        assert_eq!(HorizontalDirection::West.to_string(), "West");
    }

    #[test]
    fn display_vertical_direction() {
        assert_eq!(VerticalDirection::North.to_string(), "North");
        assert_eq!(VerticalDirection::South.to_string(), "South");
    }

    #[test]
    fn display_relative_direction() {
        assert_eq!(RelativeDirection::Left.to_string(), "Left");
        assert_eq!(RelativeDirection::Right.to_string(), "Right");
    }

    #[test]
    pub fn increment_test() {
        let mut dir = AbsoluteDirection::North;
        let mut decrement_dir = AbsoluteDirection::North;
        for _ in 0..4 {
            dir = dir.increment();
            decrement_dir = decrement_dir.decrement();
        }
        assert_eq!(dir, AbsoluteDirection::North);
        assert_eq!(decrement_dir, AbsoluteDirection::North);
    }

    #[test]
    pub fn simple_direction_test() {
        let mut dir = AbsoluteDirection::North;
        dir = dir.increment();
        assert_eq!(dir, AbsoluteDirection::West);
    }
    #[test]
    pub fn simple_direction_test_two() {
        use AbsoluteDirection::*;
        let dirs = [North, South, East, West];
        for dir in dirs {
            let mut left_dir = dir;
            //incrementing three times one way is the same as incrementing one time the other way.
            left_dir = left_dir.increment();
            left_dir = left_dir.increment();
            left_dir = left_dir.increment();
            let mut right_dir = dir;
            right_dir = right_dir.decrement();
            assert_eq!(left_dir, right_dir);
        }
    }

    #[test]
    pub fn inverse_test() {
        use AbsoluteDirection::*;
        assert_eq!(North.inverse(), South);
        assert_eq!(East.inverse(), West);
        assert_eq!(West.inverse(), East);
    }
}
