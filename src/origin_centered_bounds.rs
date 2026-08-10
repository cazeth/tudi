use crate::AbsoluteDirection;
use crate::AxisCount;
use crate::bounded::{OriginBounded, OriginCentered, OriginCenteredness};
use crate::{Bounds, bounded::Bounded};
use thiserror::Error;

/// A region that is centered around the origin.
///
/// This struct represents a region that is guaranteed to be origin-centered, which is defined as:
///
/// for both x,y-axes:
///
/// - For an axis with an odd or zero count, `max=-min`
/// - For an axis with a non-zero even count, `max=-min+1`
///
/// , the count being the number of integer points inclusively contained on the axis.
///
/// For example:
/// - `(x_min = -2, x_max = 2)` is valid since it has an odd count (5) and `x_max=-x_min`.
/// - `(x_min = -1, x_max = 2)` is valid since it has an even count (4) and `x_max=-x_min+1`.
///
#[derive(Debug, Clone, Copy)]
pub struct OriginCenteredBounds(Bounds);

impl OriginCenteredBounds {
    /// The main constructor for this struct.
    ///
    /// The method creates a origin-centered region from a x- and y-count pair.
    pub fn new(x_count: AxisCount, y_count: AxisCount) -> Self {
        let x_max = (x_count.as_u64() / 2) as i32;
        let x_min = if x_count.as_u64().is_multiple_of(2) {
            -((x_count.as_u64() / 2) as i32) + 1
        } else {
            -((x_count.as_u64() / 2) as i32)
        };

        let y_max = (y_count.as_u64() / 2) as i32;
        let y_min = if y_count.as_u64().is_multiple_of(2) {
            -((y_count.as_u64() / 2) as i32) + 1
        } else {
            -((y_count.as_u64() / 2) as i32)
        };

        Self(Bounds::from_boundaries(x_min, x_max, y_min, y_max))
    }

    /// Expand the bounds by one. Returns true if the bounds are expanded eastwards and false if expanded
    /// westwards.
    pub fn expand_bounds_horizontally(&mut self) -> bool {
        if OriginBounded::x_count(&self).as_u64().is_multiple_of(2) {
            self.0.expand_in_direction(AbsoluteDirection::West);
            false
        } else {
            self.0.expand_in_direction(AbsoluteDirection::East);
            true
        }
    }

    /// Expand the bounds by one. Returns true if the bounds are expanded northwards and false if expanded
    /// southwards.
    pub fn expand_bounds_vertically(&mut self) -> bool {
        if OriginBounded::y_count(&self).as_u64().is_multiple_of(2) {
            self.0.expand_in_direction(AbsoluteDirection::South);
            false
        } else {
            self.0.expand_in_direction(AbsoluteDirection::North);
            true
        }
    }

    pub fn x_count(&self) -> AxisCount {
        OriginBounded::x_count(self)
    }

    pub fn y_count(&self) -> AxisCount {
        OriginBounded::y_count(self)
    }
}

impl<B: Bounded> PartialEq<B> for OriginCenteredBounds {
    fn eq(&self, other: &B) -> bool {
        other.x_min_boundary() == self.x_min_boundary()
            && other.x_max_boundary() == self.x_max_boundary()
            && other.y_max_boundary() == self.y_max_boundary()
            && other.y_min_boundary() == self.y_min_boundary()
    }
}

impl TryFrom<Bounds> for OriginCenteredBounds {
    type Error = InvalidRegionError;
    fn try_from(value: Bounds) -> Result<Self, InvalidRegionError> {
        let x_min = value.x_min_boundary();
        let x_max = value.x_max_boundary();
        let y_min = value.y_min_boundary();
        let y_max = value.y_max_boundary();

        let is_origin_centered = |min: i32, max: i32| -min == max || -min + 1 == max;

        if !is_origin_centered(x_min, x_max) {
            Err(InvalidRegionError {
                min: x_min,
                max: x_max,
            })
        } else if !is_origin_centered(y_min, y_max) {
            Err(InvalidRegionError {
                min: y_min,
                max: y_max,
            })
        } else {
            Ok(OriginCenteredBounds(value))
        }
    }
}

impl OriginCenteredness for OriginCenteredBounds {
    type Distinguisher = OriginCentered;
}

impl OriginBounded for OriginCenteredBounds {
    fn x_count(&self) -> AxisCount {
        AxisCount::from_len(self.0.x_max_boundary().abs_diff(self.0.x_min_boundary()))
    }

    fn y_count(&self) -> AxisCount {
        AxisCount::from_len(self.0.y_max_boundary().abs_diff(self.0.y_min_boundary()))
    }
}

#[derive(Error, Debug)]
#[error("min {} and max {} is not a valid centered region", .min, .max)]
pub struct InvalidRegionError {
    min: i32,
    max: i32,
}

#[cfg(test)]
pub mod tests {

    use super::*;
    use crate::bounded::test::check_x_count;
    use crate::bounded::test::check_y_count;

    /// The smallest possible origin centered bounds.
    ///
    /// This should be count 1 across both dimensions and length zero.
    #[track_caller]
    fn create_smallest() -> OriginCenteredBounds {
        OriginCenteredBounds::new(
            AxisCount::from_u64_unchecked(1),
            AxisCount::from_u64_unchecked(1),
        )
    }

    #[test]
    fn valid_create_test_from_bounds() {
        assert_create_from_valid_bounds(Bounds::from_boundaries(-1, 1, -1, 1));
        assert_create_from_valid_bounds(Bounds::from_boundaries(0, 0, 0, 0));
        assert_create_from_valid_bounds(Bounds::from_boundaries(-1, 2, -1, 2));
        assert_create_from_valid_bounds(Bounds::from_boundaries(0, 1, 0, 1));
        assert_create_from_valid_bounds(Bounds::from_boundaries(0, 1, -2, 2));
        assert_create_from_valid_bounds(Bounds::from_boundaries(0, 1, -2, 3));
    }

    #[test]
    fn empty() {
        let bounds = create_smallest();
        check_x_count(&bounds, 1);
        check_y_count(&bounds, 1);
    }

    #[test]
    fn test_err_from_invalid_bounds() {
        assert_err_from_invalid_bounds(Bounds::from_boundaries(0, 3, 0, 3));
        assert_err_from_invalid_bounds(Bounds::from_boundaries(-5, 12, 0, 3));
        assert_err_from_invalid_bounds(Bounds::from_boundaries(-2, 3, -5, 0));
    }

    #[test]
    fn basic_row_expansion() {
        let mut bounds = create_smallest();
        bounds.expand_bounds_vertically();
        assert_eq!(bounds.y_max_boundary(), 1);
        assert_eq!(bounds.y_min_boundary(), 0);
        bounds.expand_bounds_vertically();
        assert_eq!(bounds.y_min_boundary(), -1);

        bounds.expand_bounds_horizontally();
        assert_eq!(bounds.x_max_boundary(), 1);
        assert_eq!(bounds.x_min_boundary(), 0);

        bounds.expand_bounds_horizontally();
        assert_eq!(bounds.x_max_boundary(), 1);
        assert_eq!(bounds.x_min_boundary(), -1);
    }

    #[test]
    fn test_row_expansion() {
        let mut bounds = create_smallest();
        let mut prev_y_min = bounds.y_min_boundary();
        let mut prev_y_max = bounds.y_max_boundary();

        for _ in 0..10 {
            bounds.expand_bounds_vertically();
            let y_min = bounds.y_min_boundary();
            let y_max = bounds.y_max_boundary();
            if prev_y_min == y_min {
                assert_eq!(prev_y_max + 1, y_max);
            } else {
                assert_eq!(prev_y_min - 1, y_min);
                assert_eq!(prev_y_max, y_max);
            }
            prev_y_min = y_min;
            prev_y_max = y_max;
        }

        assert_eq!(
            bounds,
            OriginCenteredBounds::try_from(Bounds::from_boundaries(0, 0, -5, 5)).unwrap()
        );
    }

    #[test]
    fn test_column_expansion() {
        let mut bounds = create_smallest();
        let mut prev_x_min = bounds.x_min_boundary();
        let mut prev_x_max = bounds.x_max_boundary();

        for _ in 0..10 {
            bounds.expand_bounds_horizontally();
            let x_min = bounds.x_min_boundary();
            let x_max = bounds.x_max_boundary();
            if prev_x_min == x_min {
                assert_eq!(prev_x_max + 1, x_max);
            } else {
                assert_eq!(prev_x_min - 1, x_min);
                assert_eq!(prev_x_max, x_max);
            }
            prev_x_min = x_min;
            prev_x_max = x_max;
        }

        assert_eq!(
            bounds,
            OriginCenteredBounds::try_from(Bounds::from_boundaries(-5, 5, 0, 0)).unwrap()
        );
    }

    fn assert_create_from_valid_bounds(bounds: Bounds) {
        let origin_centered_bounds = OriginCenteredBounds::try_from(bounds)
            .inspect_err(|x| println!("{x}"))
            .unwrap();

        assert_eq!(origin_centered_bounds.x_count(), bounds.x_count());
        assert_eq!(origin_centered_bounds.y_count(), bounds.y_count());
        assert_eq!(
            origin_centered_bounds.x_geometric_len(),
            bounds.x_geometric_len()
        );
        assert_eq!(
            origin_centered_bounds.y_geometric_len(),
            bounds.y_geometric_len()
        );
        assert_eq!(origin_centered_bounds.0, bounds);
    }

    fn assert_err_from_invalid_bounds(bounds: Bounds) {
        let origin_centered_bounds = OriginCenteredBounds::try_from(bounds);
        assert!(origin_centered_bounds.is_err());
    }
}
