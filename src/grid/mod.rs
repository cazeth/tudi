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
use grid_creation_error::GridCreationError;

/// A bounded two-dimensional grid that either contains an element of type T or is empty at each
/// point.
/// # Examples
/// The simplest grid is one that has an empty T-parameter. It represents a grid where each
/// coordinate is either occupied or empty and the occupied element contains no additional
/// information.
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
#[allow(unused)]
pub struct Grid<T> {
    grid_data: Vec<GridCoordinate<T>>,
    bounds: OriginCenteredBounds,
    performance_tuning: PerformanceTuning,
}
