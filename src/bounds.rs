use crate::bounded::MaybeOriginBounded;
use crate::bounded::MaybeOriginCentered;
use crate::bounded::OriginCenteredness;
//use crate::bounded::UnknownCenteredness;
use crate::AbsoluteDirection;
use crate::Coordinate;
use crate::Positioned;
#[allow(unused)] // the compiler does not realize that this crate is used because it is used
// through a blanket implementation
use crate::bounded::Bounded;

/// A bounded region.
#[derive(Debug, Clone, Copy, Hash)]
pub struct Bounds {
    northwest: Coordinate,
    southwest: Coordinate,
    northeast: Coordinate,
    southeast: Coordinate,
}

impl Bounds {
    /// This is the preferred constructor for bounds because it cannot fail.
    /// It is important to note that x_distance is not the number of coordinates on the x-axis but
    /// rather the distance between the coordinates on the axes farthest from each other. Thus, if
    /// the bounds only contains the origin (or any single point), x_len and y_len should be zero.
    pub fn new(x_min: i32, x_len: usize, y_min: i32, y_len: usize) -> Self {
        let northwest = Coordinate {
            y: y_min + y_len as i32,
            x: x_min,
        };

        let northeast = Coordinate {
            y: y_min + y_len as i32,
            x: x_min + x_len as i32,
        };

        let southeast = Coordinate {
            y: y_min,
            x: x_min + x_len as i32,
        };

        let southwest = Coordinate { y: y_min, x: x_min };
        Self {
            northwest,
            southwest,
            northeast,
            southeast,
        }
    }

    pub fn expand_in_direction(&mut self, dir: AbsoluteDirection) {
        for c in self.mut_coordinates_facing_direction(&dir) {
            c.move_in_direction(&dir, 1);
        }
    }

    pub fn add_top_row(&mut self) {
        self.northwest = self
            .northwest
            .coordinate_in_direction(AbsoluteDirection::North, 1);
        self.northeast = self
            .northeast
            .coordinate_in_direction(AbsoluteDirection::North, 1);
    }

    pub fn add_bottom_row(&mut self) {
        self.southwest = self
            .southwest
            .coordinate_in_direction(AbsoluteDirection::South, 1);
        self.southeast = self
            .southeast
            .coordinate_in_direction(AbsoluteDirection::South, 1);
    }

    fn mut_coordinates_facing_direction(
        &mut self,
        dir: &AbsoluteDirection,
    ) -> [&mut Coordinate; 2] {
        use AbsoluteDirection::*;
        match dir {
            North => [&mut self.northwest, &mut self.northeast],
            South => [&mut self.southeast, &mut self.southwest],
            East => [&mut self.southeast, &mut self.northeast],
            West => [&mut self.northwest, &mut self.southwest],
        }
    }
}

impl<B: Bounded> PartialEq<B> for Bounds {
    fn eq(&self, other: &B) -> bool {
        other.x_min_boundary() == self.x_min_boundary()
            && other.x_max_boundary() == self.x_max_boundary()
            && other.y_max_boundary() == self.y_max_boundary()
            && other.y_min_boundary() == self.y_min_boundary()
    }
}

impl OriginCenteredness for Bounds {
    type Distinguisher = MaybeOriginCentered;
}

impl MaybeOriginBounded for Bounds {
    fn x_min(&self) -> i32 {
        self.southwest.x
    }

    fn x_max(&self) -> i32 {
        self.southeast.x
    }

    fn y_min(&self) -> i32 {
        self.southwest.y
    }

    fn y_max(&self) -> i32 {
        self.northeast.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let bounds = Bounds::new(0, 0, 0, 0);
        assert_eq!(bounds.northwest_corner(), Coordinate::default());
        assert_eq!(bounds.southwest_corner(), Coordinate::default());
        assert_eq!(bounds.northeast_corner(), Coordinate::default());
        assert_eq!(bounds.southeast_corner(), Coordinate::default());
        assert_eq!(bounds.x_count(), 1);
        assert_eq!(bounds.y_count(), 1);
    }

    #[test]
    fn add_row_test() {
        let mut bounds = Bounds::new(-10, 1, -10, 2);
        assert_eq!(bounds.y_count(), 3);
        bounds.add_top_row();
        bounds.add_bottom_row();
        assert_eq!(bounds.x_count(), 2);
        assert_eq!(bounds.y_count(), 5);
    }

    #[test]
    fn expansion_test() {
        let mut bounds = Bounds::new(0, 0, 0, 0);
        assert_eq!(bounds.y_count(), 1);
        bounds.expand_in_direction(AbsoluteDirection::North);
        assert_eq!(bounds.y_count(), 2);
        bounds.expand_in_direction(AbsoluteDirection::South);
        assert_eq!(bounds.y_count(), 3);

        assert_eq!(bounds.x_count(), 1);
        bounds.expand_in_direction(AbsoluteDirection::East);
        assert_eq!(bounds.x_count(), 2);
    }
}
