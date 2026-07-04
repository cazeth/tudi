mod clone_grid;
mod generic_grid;
mod grid_coordinate;
mod grid_creation_error;
mod grid_error;
mod grid_iter;
mod performance_tuning;
pub use self::grid_error::GridError;
use self::performance_tuning::PerformanceTuning;
use crate::OriginCenteredBounds;
use grid_coordinate::GridCoordinate;
pub use grid_creation_error::GridCreationError;

/// A bounded two-dimensional grid that either contains an element of type T or is empty at each
/// point.
///
/// # Overview
///
/// The Grid constitutes one of the key structures of this crate. In many cases, it is a suitable
/// alternative to model anything two-dimensional.
///
/// The grid has the following properties:
///
/// - A grid occupies a finite two-dimensional space; it has boundaries across the x and y dimension.
///
/// - A grid has an x- and y-count. This is the number of coordinates across each dimension.
///
/// - A grid has an x- and y-length. This is the distance between the greatest and smallest coordinate
/// along each axis. For each dimension, `length = count - 1`.
///
/// - A grid is origin-centered; the finite space that it occupies has the origin as its center. The grid always upholds this property, exactly or as closely as possible. If it is not exactly possible, because of an even coordinate count along either the x- or y-dimension, a grid is biased toward the positive side. For instance, a grid with a 4x4 count has the boundaries `x_min = -1, x_max = 2, y_min = -1, y_max = 2`. See also [OriginCenteredBounds].
///
/// - A grid is either empty or contains a single element T at each coordinate.
///
/// # Examples
///
/// The simplest grid is one that has a unit type parameter. It represents a grid where each
/// coordinate is either occupied or empty, and the occupied element carries no additional
/// information - it only acts as a marker.
///
/// ```
/// use tudi::Grid;
/// use tudi::Coordinate;
///
/// let mut grid = Grid::new(3, 3); // A 3x3 grid.
/// grid.element_statuses(); // all elements are empty
/// grid.store_element(&Coordinate::default(),()); // store an element at the origin.
/// grid.element_statuses(); // the grid now contains an element at the origin.
///
/// assert!(grid.element(&Coordinate::default()).is_ok()); // the grid now contains an element at
/// // the origin.
/// ```
#[derive(Debug)]
pub struct Grid<T> {
    grid_data: Vec<GridCoordinate<T>>,
    bounds: OriginCenteredBounds,
    performance_tuning: PerformanceTuning,
}
