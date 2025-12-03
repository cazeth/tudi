use crate::AbsoluteDirection;
use crate::bounded::{OriginBounded, OriginCentered, OriginCenteredness};
use crate::{Bounds, bounded::Bounded};
use thiserror::Error;

/// A region that is centered around the origin.
///
/// This struct represents a region that is guaranteed to withold the property of origin-centeredness, which is defined as:
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
    pub fn new(x_count: u64, y_count: u64) -> Self {
        let x_min = -(x_count as i32 - ((x_count + 1) % 2) as i32) / 2;
        let x_max = (x_count as i32 + ((x_count + 1) % 2) as i32) / 2;
        let y_min = -(y_count as i32 - ((y_count + 1) % 2) as i32) / 2;
        let y_max = (y_count as i32 + ((y_count + 1) % 2) as i32) / 2;

        let x_dist = x_max - x_min;
        let y_dist = y_max - y_min;

        let bounds = Bounds::new(x_min, x_dist as usize, y_min, y_dist as usize);
        Self::try_from(bounds).unwrap()
    }

    /// Expand the bounds by one. Returns true if the bounds are expanded westwards and false if expanded
    /// eastwards.
    pub fn expand_bounds_horizontally(&mut self) -> bool {
        if OriginBounded::x_count(&self) % 2 == 0 {
            self.0.expand_in_direction(AbsoluteDirection::West);
            true
        } else {
            self.0.expand_in_direction(AbsoluteDirection::East);
            false
        }
    }

    /// Expand the bounds by one. Returns true if the bounds are expanded northwards and false if expanded
    /// southwards.
    pub fn expand_bounds_vertically(&mut self) -> bool {
        if OriginBounded::y_count(&self) % 2 == 0 {
            self.0.expand_in_direction(AbsoluteDirection::South);
            false
        } else {
            self.0.expand_in_direction(AbsoluteDirection::North);
            true
        }
    }

    pub fn x_count(&self) -> usize {
        OriginBounded::x_count(self)
    }

    pub fn y_count(&self) -> usize {
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
        let x_max = value.x_min_boundary() + value.x_geometric_len() as i32;
        let y_max = value.y_min_boundary() + value.y_geometric_len() as i32;
        let x_min = value.x_min_boundary();
        let y_min = value.y_min_boundary();

        let x_dist = (x_max - x_min) as usize;
        let y_dist = (y_max - y_min) as usize;

        if !(-x_min == x_max || -x_min + 1 == x_max) {
            Err(InvalidRegionError {
                min: x_min,
                max: x_max,
            })
        } else if !(-y_min == y_max || -y_min + 1 == y_max) {
            Err(InvalidRegionError {
                min: y_min,
                max: y_max,
            })
        } else {
            let bounds = Bounds::new(x_min, x_dist, y_min, y_dist);
            Ok(OriginCenteredBounds(bounds))
        }
    }
}

impl OriginCenteredness for OriginCenteredBounds {
    type Distinguisher = OriginCentered;
}

impl OriginBounded for OriginCenteredBounds {
    fn x_count(&self) -> usize {
        (self.0.x_max_boundary() - self.0.x_min_boundary() + 1)
            .try_into()
            .unwrap()
    }

    fn y_count(&self) -> usize {
        (self.0.y_max_boundary() - self.0.y_min_boundary() + 1)
            .try_into()
            .unwrap()
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
    #[test]
    fn valid_create_test_from_bounds() {
        assert_create_from_valid_bounds(Bounds::new(-1, 2, -1, 2));
        assert_create_from_valid_bounds(Bounds::new(0, 0, 0, 0));
        assert_create_from_valid_bounds(Bounds::new(-1, 3, -1, 3));
        assert_create_from_valid_bounds(Bounds::new(0, 1, 0, 1));
        assert_create_from_valid_bounds(Bounds::new(0, 1, -2, 4));
        assert_create_from_valid_bounds(Bounds::new(0, 1, -2, 5));
    }

    #[test]
    fn test_err_from_invalid_bounds() {
        assert_err_from_invalid_bounds(Bounds::new(0, 3, 0, 3));
        assert_err_from_invalid_bounds(Bounds::new(-5, 12, 0, 3));
        assert_err_from_invalid_bounds(Bounds::new(-2, 3, -5, 0));
    }

    #[test]
    fn basic_row_expansion() {
        let mut bounds = OriginCenteredBounds::new(0, 0);
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
        let mut bounds = OriginCenteredBounds::try_from(Bounds::new(0, 0, 0, 0)).unwrap();
        for _ in 0..10 {
            bounds.expand_bounds_vertically();
        }

        assert_eq!(
            bounds,
            OriginCenteredBounds::try_from(Bounds::new(0, 0, -5, 10)).unwrap()
        );
    }

    fn assert_create_from_valid_bounds(bounds: Bounds) {
        let origin_centered_bounds = OriginCenteredBounds::try_from(bounds)
            .inspect_err(|x| println!("{x}"))
            .unwrap();
        assert_eq!(origin_centered_bounds.0, bounds);
    }

    fn assert_err_from_invalid_bounds(bounds: Bounds) {
        let origin_centered_bounds = OriginCenteredBounds::try_from(bounds);
        assert!(origin_centered_bounds.is_err());
    }
}
