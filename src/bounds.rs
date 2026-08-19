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
    /// It is important to note that x_len is not the number of coordinates on the x-axis but
    /// rather the distance between the coordinates on the axes farthest from each other. Thus, if
    /// the bounds only contains the origin (or any single point), x_len and y_len should be zero.
    #[deprecated(
        since = "0.3.0",
        note = "Since this method is not actually infallible, it is preferred to use fn from_boundaries instead."
    )]
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

    /// Creates bounds from two boundaries along each axis.
    ///
    /// The boundary arguments may be provided in either order.
    pub fn from_boundaries(
        first_x_boundary: i32,
        second_x_boundary: i32,
        first_y_boundary: i32,
        second_y_boundary: i32,
    ) -> Self {
        let x_min = first_x_boundary.min(second_x_boundary);
        let x_max = first_x_boundary.max(second_x_boundary);
        let y_min = first_y_boundary.min(second_y_boundary);
        let y_max = first_y_boundary.max(second_y_boundary);

        Self {
            northwest: Coordinate { x: x_min, y: y_max },
            southwest: Coordinate { x: x_min, y: y_min },
            northeast: Coordinate { x: x_max, y: y_max },
            southeast: Coordinate { x: x_max, y: y_min },
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
    use crate::bounded::test::check_x_count;
    use crate::bounded::test::check_x_len;
    use crate::bounded::test::check_x_max;
    use crate::bounded::test::check_x_min;
    use crate::bounded::test::check_y_count;
    use crate::bounded::test::check_y_len;
    use crate::bounded::test::check_y_max;
    use crate::bounded::test::check_y_min;

    #[test]
    fn new() {
        #[expect(deprecated)]
        let bounds = Bounds::new(0, 0, 0, 0);
        assert_eq!(bounds.northwest_corner(), Coordinate::default());
        assert_eq!(bounds.southwest_corner(), Coordinate::default());
        assert_eq!(bounds.northeast_corner(), Coordinate::default());
        assert_eq!(bounds.southeast_corner(), Coordinate::default());
        check_x_count(bounds, 1);
        check_y_count(bounds, 1);
    }

    #[test]
    fn from_boundaries_normalizes_boundary_order() {
        let bounds = Bounds::from_boundaries(3, -2, 7, -5);

        check_x_min(bounds, -2);
        check_x_max(bounds, 3);
        check_y_min(bounds, -5);
        check_y_max(bounds, 7);
    }

    #[test]
    fn from_boundaries_accepts_full_i32_span() {
        let bounds = Bounds::from_boundaries(i32::MIN, i32::MAX, i32::MAX, i32::MIN);

        check_x_min(bounds, i32::MIN);
        check_x_max(bounds, i32::MAX);
        check_y_min(bounds, i32::MIN);
        check_y_max(bounds, i32::MAX);
    }

    #[test]
    fn geometric_x_length_handles_full_coord_span() {
        let bounds = Bounds {
            northwest: Coordinate {
                x: i32::MIN + 1,
                y: 0,
            },
            southwest: Coordinate {
                x: i32::MIN + 1,
                y: 0,
            },
            northeast: Coordinate { x: i32::MAX, y: 0 },
            southeast: Coordinate { x: i32::MAX, y: 0 },
        };

        check_x_len(bounds, (i32::MAX + i32::MIN - 1) as u32);
    }

    #[test]
    fn geometric_y_length_handles_full_coord_span() {
        let bounds = Bounds {
            northwest: Coordinate { y: i32::MAX, x: 0 },
            southwest: Coordinate {
                y: i32::MIN + 1,
                x: 0,
            },
            northeast: Coordinate { y: i32::MAX, x: 0 },
            southeast: Coordinate {
                y: i32::MIN + 1,
                x: 0,
            },
        };

        let widened_min: i64 = (i32::MIN + 1) as i64;

        let widened_max: i64 = i32::MAX as i64;

        let geometric_len: u64 = (widened_max - widened_min) as u64;
        check_y_len(bounds, geometric_len as u32);
    }

    #[test]
    fn x_coordinate_count_supports_full_coordinate_span() {
        let bounds = Bounds::from_boundaries(i32::MIN + 1, i32::MAX, 0, 0);
        check_x_count(bounds, u32::MAX as u64);
    }

    #[test]
    fn y_coordinate_count_supports_full_coordinate_span() {
        let bounds = Bounds::from_boundaries(0, 0, i32::MIN + 1, i32::MAX);
        check_y_count(bounds, u32::MAX as u64);
    }

    #[test]
    fn max_x_boundary_produces_correct_x_count() {
        let bounds = Bounds::from_boundaries(0, i32::MAX, 0, 0);
        let expected_x_count = u64::try_from(i32::MAX).unwrap().checked_add(1).unwrap();
        check_x_count(bounds, expected_x_count);
    }

    #[test]
    fn max_y_boundary_produces_correct_y_count() {
        let bounds = Bounds::from_boundaries(0, 0, 0, i32::MAX);
        let expected_y_count = u64::try_from(i32::MAX).unwrap().checked_add(1).unwrap();
        check_y_count(bounds, expected_y_count);
    }

    #[test]
    fn longest_y_boundary_produces_correct_y_count() {
        let bounds = Bounds::from_boundaries(0, 0, i32::MIN + 1, i32::MAX);
        let expected_y_count = u64::from(i32::MAX.abs_diff(i32::MIN + 1))
            .checked_add(1)
            .unwrap();
        check_y_count(bounds, expected_y_count);
    }

    #[test]
    fn longest_x_boundary_produces_correct_x_count() {
        let bounds = Bounds::from_boundaries(i32::MIN + 1, i32::MAX, 0, 0);
        let expected_x_count = u64::from(i32::MAX.abs_diff(i32::MIN + 1))
            .checked_add(1)
            .unwrap();
        check_x_count(bounds, expected_x_count);
    }

    #[test]
    fn add_row_test() {
        #[expect(deprecated)]
        let mut bounds = Bounds::new(-10, 1, -10, 2);
        check_y_count(bounds, 3);
        bounds.add_top_row();
        bounds.add_bottom_row();
        check_x_count(bounds, 2);
        check_y_count(bounds, 5);
    }

    #[test]
    fn expansion_test() {
        #[expect(deprecated)]
        let mut bounds = Bounds::new(0, 0, 0, 0);
        check_y_count(bounds, 1);
        bounds.expand_in_direction(AbsoluteDirection::North);
        check_y_count(bounds, 2);
        bounds.expand_in_direction(AbsoluteDirection::South);
        check_y_count(bounds, 3);

        check_x_count(bounds, 1);
        bounds.expand_in_direction(AbsoluteDirection::East);
        check_x_count(bounds, 2);
    }

    macro_rules! check_to_grid_like {
        ([$nw:expr, $se:expr] with [$x:expr, $y:expr] is out of bounds) => {
            let lengths = [$x, $y];
            let bounds = Bounds::from_boundaries($nw.0, $se.0, $nw.1, $se.1);
            assert!(bounds.to_grid_like(lengths).is_err())
        };

        ([$nw:expr, $se:expr] with [$x:expr, $y:expr] has out of bounds pos ($x_err:expr, $y_err:expr)) => {
            let lengths = [$x, $y];
            let bounds = Bounds::from_boundaries($nw.0, $se.0, $nw.1, $se.1);
            let res = bounds.to_grid_like(lengths);
            if let Err(bounds_err) = res {
                assert_eq!(bounds_err.position().x, $x_err);
                assert_eq!(bounds_err.position().y, $y_err);
            } else {
                panic!("expected out of bounds but wasn't!");
            }
        };
        ([$nw:expr, $se:expr] with [$x:expr, $y:expr] is within bounds) => {
            let lengths = [$x, $y];
            let bounds = Bounds::from_boundaries($nw.0, $se.0, $nw.1, $se.1);
            assert!(bounds.to_grid_like(lengths).is_ok())
        };
    }

    #[test]
    fn to_grid_like_out_of_bounds() {
        check_to_grid_like!([(0,0), (0,0)] with [5,5] has out of bounds pos (5,-5));
        check_to_grid_like!([(i32::MIN+1, i32::MAX), (0, 0)] with [u32::MAX -1,u32::MAX-1] has out of bounds pos (i32::MAX, i32::MIN+1));
        check_to_grid_like!([(i32::MAX, i32::MIN+1), (i32::MAX, i32::MIN+1)] with [1,1] has out of bounds pos (i32::MAX, i32::MIN+1));
        check_to_grid_like!([(i32::MAX, i32::MIN+1), (i32::MAX, i32::MIN+1)] with [0,0] is within bounds);
        check_to_grid_like!([(0,0), (0,0)] with [0,0] is within bounds);
        check_to_grid_like!([(0,0), (0,0)] with [1,1] has out of bounds pos (1, -1));
        check_to_grid_like!([(0,0), (10,-10)] with [5,5] is within bounds);
        check_to_grid_like!([(-5,5), (5, -5)] with [100,0] is out of bounds);
    }
}
