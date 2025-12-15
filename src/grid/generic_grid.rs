use super::Grid;
use super::grid_coordinate::GridCoordinate;
use super::grid_iter::GridIter;
use super::performance_tuning::PerformanceTuning;
use crate::AbsoluteDirection;
use crate::BoundedMovingObject;
use crate::Bounds;
use crate::Coordinate;
use crate::GridError;
use crate::OriginCenteredBounds;
use crate::OutOfBoundsError;
use crate::Positioned;
use crate::bounded::Bounded;
use crate::bounded::OriginBounded;
use crate::bounded::OriginCentered;
use crate::bounded::OriginCenteredness;
use crate::grid::GridCreationError;
use itertools::iproduct;

impl<T> Grid<T> {
    ///Create a rectangular grid with empty elements.
    ///The x and y-variables represent the number of coordinates along an axis, rather than the
    ///length between the extremes along the axis. For instance, x_count = 3 creates a grid that
    ///contains the coordinates with x= -1, x = 0, and x = 1.
    ///# Examples
    /// ```
    /// use tudi::Grid;
    /// use tudi::bounded::Bounded;
    /// let x_count = 3;
    /// let y_count = 3;
    /// let grid : Grid<()> = Grid::new(x_count,y_count);
    /// assert_eq!(grid.x_count(), x_count);
    /// assert_eq!(grid.y_count(), y_count);
    /// assert_eq!(grid.iter_new().count(), x_count*y_count); // There are nine coordinates
    /// assert_eq!(grid.iter_elements_new().count(), 0); // There are zero elements
    /// ```
    /// ```
    /// use tudi::Grid;
    /// use tudi::Coordinate;
    /// use tudi::bounded::Bounded;
    /// let grid : Grid<()> = Grid::new(6, 6);  // Returns an empty 6x6 grid.
    /// // A Grid<()> cannot contain anything (because of the unit type parameter), so you will likely want to replace the type
    /// // with something more useful.
    ///
    /// assert_eq!(grid.iter_elements_new().count(), 0); // The grid contains no elements
    /// assert_eq!(grid.iter_new().count(), 36);         // but it contains 36 coordinates.
    /// assert_eq!(grid.x_max_boundary(), 3);         // the grid is centered around the origin
    ///                                              // with a bias toward the positive axes.
    /// assert_eq!(grid.x_min_boundary(), -2);
    /// assert_eq!(grid.y_max_boundary(), 3);
    /// assert_eq!(grid.y_min_boundary(), -2);
    /// ```
    ///
    pub fn new(x_count: usize, y_count: usize) -> Self {
        let bounds = OriginCenteredBounds::new(x_count as u64, y_count as u64);
        let mut result = Self {
            grid_data: Vec::new(),
            bounds,
            performance_tuning: PerformanceTuning::Auto,
        };

        for (y, x) in iproduct!(
            (result.y_min_boundary()..=result.y_max_boundary()).rev(),
            result.x_min_boundary()..=result.x_max_boundary()
        ) {
            result
                .grid_data
                .push(GridCoordinate::Empty(Coordinate { x, y }));
        }
        result
    }

    /// Create a new empty grid with the same bounds as another OriginCenteredBounded.
    ///
    /// # Examples
    ///
    /// ```
    /// use tudi::Grid;
    /// use tudi::OriginCenteredBounds;
    /// use tudi::Bounded;
    /// let bounds = OriginCenteredBounds::new(4,4);
    /// let grid : Grid<()> = Grid::from_bounds(&bounds);
    /// assert_eq!(grid.x_max_boundary(), 2);
    /// assert_eq!(grid.x_min_boundary(), -1);
    /// assert_eq!(grid.y_max_boundary(), 2);
    /// assert_eq!(grid.y_min_boundary(), -1);
    ///
    /// ```
    // note : We could also implement the From trait here, but the Grid itself implements
    // OriginBounded, so this implement is not allowed since it conflicts with an already existing
    // blanket implementation.
    pub fn from_bounds<B: OriginBounded>(other: &B) -> Self {
        Self::new(other.x_count(), other.y_count())
    }

    pub fn bounds(&self) -> OriginCenteredBounds {
        self.bounds
    }

    /// Get a reference to an element in the Grid.
    ///
    ///
    /// # Panics
    /// This method panics if the coordinate is out of bounds.
    ///
    /// See also [`checked_element`](Grid::checked_element())
    ///
    /// # Examples
    /// ```
    /// use tudi::Grid;
    /// use tudi::bounded::Bounded;
    /// use tudi::Coordinate;
    /// let mut grid = Grid::new(3, 3); // create a new 3x3 grid.
    /// grid.store_element(&Coordinate::default(), 1); // store 1 at the origin.
    /// assert!(grid.element(&Coordinate::default()).is_ok()); // This method returns a borrowed
    /// // version of the element.
    /// assert!(grid.element(&grid.northeast_corner()).is_err());
    ///
    /// ```
    pub fn element_unchecked<C: Positioned>(&self, coordinate: &C) -> Option<&T> {
        assert!(self.is_within_bounds(coordinate));
        let index = self.coordinate_to_index(coordinate).unwrap();
        let val = &self.grid_data[index];

        if let GridCoordinate::Object(element) = val {
            Some(element)
        } else {
            None
        }
    }

    pub fn element<C: Positioned>(&self, coordinate: &C) -> Result<&T, GridError> {
        if !self.is_within_bounds(coordinate) {
            Err(GridError::OutOfBoundsError(OutOfBoundsError::new(
                *coordinate.position(),
            )))
        } else {
            self.element_unchecked(coordinate)
                .ok_or_else(|| GridError::UnoccupiedError(*coordinate.position()))
        }
    }

    pub fn get_mut_element<C: Positioned>(&mut self, coordinate: &C) -> Result<&mut T, GridError> {
        assert!(self.is_within_bounds(coordinate));

        let index = self.coordinate_to_index(coordinate)?;
        let val = &mut self.grid_data[index];

        if let GridCoordinate::Object(element) = val {
            Ok(element)
        } else {
            Err(GridError::UnoccupiedError(*coordinate.position()))
        }
    }

    pub fn store_element<C: Positioned>(
        &mut self,
        coordinate: &C,
        element: T,
    ) -> Result<Option<T>, GridError> {
        let index = self.coordinate_to_index(coordinate)?;
        let previous_val =
            std::mem::replace(&mut self.grid_data[index], GridCoordinate::Object(element));
        match previous_val {
            GridCoordinate::Object(val) => Ok(Some(val)),
            GridCoordinate::Empty(_) => Ok(None),
        }
    }

    // Returns error when there is no element at a coordinate at which this function is called.
    pub fn remove_element<C: Positioned>(&mut self, coordinate: &C) -> Result<T, GridError> {
        let index = self.coordinate_to_index(coordinate)?;
        let previous_grid_coordinate = std::mem::replace(
            &mut self.grid_data[index],
            GridCoordinate::Empty(*coordinate.position()),
        );

        if let GridCoordinate::Object(val) = previous_grid_coordinate {
            self.grid_data[index] = GridCoordinate::Empty(*coordinate.position());
            Ok(val)
        } else {
            Err(GridError::UnoccupiedError(*coordinate.position()))
        }
    }

    pub fn iter_new(&self) -> GridIter<T> {
        match self.performance_tuning {
            PerformanceTuning::Auto => self.iter_new_memory(),

            PerformanceTuning::Speed => self.iter_new_speed(),

            PerformanceTuning::Memory => self.iter_new_memory(),
        }
    }

    fn iter_new_memory(&self) -> GridIter<T> {
        GridIter::new(self)
    }

    fn iter_new_speed(&self) -> GridIter<T> {
        GridIter::new(self)
    }

    pub fn iter_mut_new(&mut self) -> impl Iterator<Item = (Coordinate, Option<&mut T>)> {
        let coordinates = (0..OriginBounded::x_count(&self) * OriginBounded::y_count(&self))
            .map(|x| self.index_to_coordinate(x).unwrap())
            .collect::<Vec<Coordinate>>();

        self.grid_data
            .iter_mut()
            .enumerate()
            .map(move |(index, grid_coordinate)| match grid_coordinate {
                GridCoordinate::Object(val) => (coordinates[index], Some(val)),
                GridCoordinate::Empty(_) => (coordinates[index], None),
            })
    }

    pub fn iter_mut_elements_new(&mut self) -> impl Iterator<Item = (Coordinate, &mut T)> {
        self.iter_mut_new()
            .filter(|(_, grid_coordinate)| grid_coordinate.is_some())
            .map(|(coord, element)| (coord, element.unwrap()))
    }

    pub fn iter_elements_new(&self) -> impl Iterator<Item = (Coordinate, &T)> {
        self.iter_new()
            .filter(|(_, grid_coordinate)| grid_coordinate.is_some())
            .map(|(coord, element)| (coord, element.unwrap()))
    }

    /// returns a vec of all empty rows.
    /// It starts at the bottom (with negative indices).
    pub fn empty_rows(&self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for y in self.y_min_boundary()..=self.y_max_boundary() {
            'inner: for x in self.x_min_boundary()..=self.x_max_boundary() {
                if self.element_unchecked(&Coordinate { x, y }).is_some() {
                    break 'inner;
                } else if x == self.x_max_boundary() {
                    result.push(y);
                }
            }
        }

        result
    }

    /// returns a vec of all empty columns
    pub fn empty_columns(&self) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for x in self.x_min_boundary()..=self.x_max_boundary() {
            //println!("y: {}", y);
            'inner: for y in self.y_min_boundary()..=self.y_max_boundary() {
                //println!("x: {}", x);
                if self.element_unchecked(&Coordinate { x, y }).is_some() {
                    //println!("Coordinate x: {x}, y : {y} is some!");
                    break 'inner;
                } else if y == self.y_max_boundary() {
                    result.push(x);
                }
            }
        }

        result
    }

    pub fn move_elements_above_row_in_direction(
        &mut self,
        y_coord: i32,
        direction: AbsoluteDirection,
    ) -> Result<(), GridError> {
        self.row_filter_move_elements_in_direction(Coordinate::is_above_row, y_coord, direction)
    }

    pub fn move_elements_below_row_in_direction(
        &mut self,
        y_coord: i32,
        direction: AbsoluteDirection,
    ) -> Result<(), GridError> {
        self.row_filter_move_elements_in_direction(Coordinate::is_below_row, y_coord, direction)
    }

    /// The count along the x-dimension.
    /// # Examples
    /// ```
    /// use tudi::Grid;
    ///
    /// let grid: Grid<()> = Grid::new(3, 3);
    /// assert_eq!(grid.x_count(), 3);
    ///
    /// ```
    pub fn x_count(&self) -> usize {
        OriginBounded::x_count(self)
    }

    /// The count along the y-dimension.
    /// # Examples
    /// ```
    /// use tudi::Grid;
    ///
    /// let grid: Grid<()> = Grid::new(3, 3);
    /// assert_eq!(grid.y_count(), 3);
    ///
    /// ```
    pub fn y_count(&self) -> usize {
        OriginBounded::y_count(self)
    }

    /// move an element within the grid by a direction.
    /// Return an error if:
    /// the resulting move would be out of bounds.
    /// the resulting move would result in a collision.
    /// the coordinate does not contain an element
    pub fn move_element_in_direction(
        &mut self,
        coordinate: &Coordinate,
        direction: AbsoluteDirection,
    ) -> Result<Coordinate, GridError> {
        let mut marker = BoundedMovingObject::try_from((&self, coordinate))?;

        if marker.move_in_absolute_direction(direction, 1) {
            if self.element_unchecked(marker.position()).is_some() {
                return Err(GridError::CollisionError);
            }

            let element = self.remove_element(coordinate)?;
            self.store_element(marker.position(), element)?;
            Ok(*marker.position())
        } else {
            Err(GridError::OutOfBoundsError(OutOfBoundsError::new(
                marker.coordinate_in_direction(direction, 1),
            )))
        }
    }

    /// Expand the grid at a row. The Grid remains OriginCentered.
    /// You can think of the y_coord of an indicator of which elements are pushed.
    /// This, along with if the grid is odd or even numbered, determines which elements are moved.
    /// The movement of each element can be summarized as follows:
    ///
    /// |Element pos | Even #rows | Odd #rows |
    /// |------------|------------|-----------|
    /// | **above y**    | no change  | moved up  |
    /// |    **at y**    | moved down | moved up  |
    /// | **below y**    | moved down | no change |
    ///
    pub fn expand_at_row(&mut self, y_coord: i32) -> bool {
        let top_add = self.add_row();

        if top_add {
            self.move_elements_above_row_in_direction(y_coord, AbsoluteDirection::North)
                .unwrap();
            true
        } else {
            self.move_elements_below_row_in_direction(y_coord, AbsoluteDirection::South)
                .unwrap();
            false
        }
    }

    /// Adds an empty row to the grid. If the grid has an even number of rows it always has one
    /// more positive row than negative row, and if the grid has an odd number of rows the positive
    /// and negative number of rows are equal. This function preserves this property.
    /// If the row is added to the top it returns true otherwise it returns false.
    /// # Examples
    /// ```
    /// use tudi::Grid;
    /// use tudi::Bounded;
    /// let mut grid : Grid<()> = Grid::new(5, 5);
    ///
    /// // since the grid has an odd number of rows, a new row is added to the top.
    /// assert_eq!(grid.y_max_boundary(), 2);
    /// assert_eq!(grid.y_min_boundary(), -2);
    /// grid.add_row();
    /// assert_eq!(grid.y_max_boundary(), 3);
    /// assert_eq!(grid.y_min_boundary(), -2);
    ///
    /// grid.add_row();
    /// assert_eq!(grid.y_max_boundary(), 3);
    /// assert_eq!(grid.y_min_boundary(), -3);
    ///
    /// ```
    pub fn add_row(&mut self) -> bool {
        if OriginBounded::y_count(&self) % 2 == 0 {
            self.add_bottom_row();
            false
        } else {
            self.add_top_row();
            true
        }
    }

    /// Adds an empty bottom row to the grid. The reason that this function isn't public is because
    /// the grid always maintains the origin as its center with a bias toward the positive
    /// coordinates and that assumption would break if this function were public.
    /// This function should only be called when the number of rows is even, in order to maintain
    /// the centering around the origin.
    fn add_bottom_row(&mut self) {
        let y_min = self.y_min_boundary() - 1;
        for x in self.x_min_boundary()..=self.x_max_boundary() {
            self.grid_data
                .push(GridCoordinate::Empty(Coordinate { x, y: y_min }));
        }
        self.bounds.expand_bounds_vertically();
    }

    /// Adds an empty top row to the grid. The reason that this function isn't public is because
    /// the grid always maintains the origin as its center with a bias toward the positive
    /// coordinates and that assumption would break if this function were public.
    /// This function should only be called when the number of rows is even, in order to maintain
    /// the centering around the origin.
    fn add_top_row(&mut self) {
        let y_max = self.y_max_boundary() + 1;
        for x in (self.x_min_boundary()..=self.x_max_boundary()).rev() {
            self.grid_data
                .insert(0, GridCoordinate::Empty(Coordinate { x, y: y_max }));
        }
        self.bounds.expand_bounds_vertically();
    }

    /// This method does two things:
    ///
    /// I: It filters out the coordinates that do not adhere to a
    /// provided criterion. The filter takes a coordinate and an i32 that represents a row.
    ///
    /// II: It moves all the coordinates in the provided direction.
    ///
    /// The method returns an error in case of out of bounds or collision.
    fn row_filter_move_elements_in_direction(
        &mut self,
        filter: fn(&Coordinate, i32) -> bool,
        row: i32,
        direction: AbsoluteDirection,
    ) -> Result<(), GridError> {
        let element_coordinates = self
            .iter_elements_new()
            .map(|(coordinate, _)| coordinate)
            .filter(|c| filter(c, row))
            .collect::<Vec<Coordinate>>();

        if direction == AbsoluteDirection::South || direction == AbsoluteDirection::East {
            for c in element_coordinates.iter().rev() {
                self.move_element_in_direction(c, direction)?;
            }
        } else if direction == AbsoluteDirection::North || direction == AbsoluteDirection::West {
            for c in element_coordinates.iter() {
                self.move_element_in_direction(c, direction)?;
            }
        }

        Ok(())
    }

    /// Transpose a grid. Changes the size of an NxM grid to MxN and moves all elements from [i][j]
    /// to [j][i] (defined as matrix-like coordinates rather than grid-like).
    /// # Examples
    /// ```
    /// use tudi::Grid;
    /// use tudi::Coordinate;
    /// let mut grid = Grid::new(3,2);
    /// grid.store_element(&Coordinate{x: 1, y : 0}, () ); // [2][1] in matrix-like coordinates.
    /// grid.transpose_new();
    /// assert!(grid.element(&Coordinate{y : -1, x: 1}).is_ok()); // [1][2] in matrix-like
    /// // coordinates
    ///
    /// ```
    ///
    pub fn transpose_new(&mut self) {
        let old_grid = std::mem::replace(
            self,
            Self::new(OriginBounded::y_count(&self), OriginBounded::x_count(&self)),
        );

        let previous_bounds = Bounds::new(
            old_grid.x_min_boundary(),
            old_grid.x_geometric_len(),
            old_grid.y_min_boundary(),
            old_grid.y_geometric_len(),
        );

        for (coordinate, element) in old_grid.into_iter() {
            let matrix_coordinates = previous_bounds.to_matrix_like(&coordinate);
            let new_coordinate = self
                .to_grid_like([matrix_coordinates[1], matrix_coordinates[0]])
                .unwrap();

            if let Some(e) = element {
                self.store_element(&new_coordinate, e)
                    .expect("should never fail");
            }
        }
    }

    pub fn print_properties(&self) {
        println!("-----");
        println!("y min is {}", self.y_min_boundary());
        println!("y max is {}", self.y_max_boundary());
        println!("x_min is {}", self.x_min_boundary());
        println!("x_max is {}", self.x_max_boundary());
        println!("number of elements is {}", self.iter_elements_new().count());
        println!("-----");
    }

    /// A string where '#' marks a occupied element and '.' marks an empty element. with one line for
    /// each row in the grid.
    /// A simple way to quickly see what is going on in a small grid.
    pub fn element_statuses(&self) -> String {
        let mut result = String::with_capacity((self.x_count() + 1) * self.y_count());
        for (index, element) in self.iter_new() {
            if element.is_some() {
                result.push('#');
            } else {
                result.push('.')
            };

            if index.x_coordinate() == self.x_max_boundary()
                && index.y_coordinate() != self.y_min_boundary()
            {
                result.push('\n');
            }
        }
        result
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = (Coordinate, Option<T>);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> std::vec::IntoIter<Self::Item> {
        let bounds = Bounds::new(
            self.x_min_boundary(),
            self.x_geometric_len(),
            self.y_min_boundary(),
            self.y_geometric_len(),
        );

        self.grid_data
            .into_iter()
            .enumerate()
            .map(move |(index, grid_coordinate)| match grid_coordinate {
                GridCoordinate::Object(val) => {
                    (bounds.index_to_coordinate(index).unwrap(), Some(val))
                }
                GridCoordinate::Empty(_) => (bounds.index_to_coordinate(index).unwrap(), None),
            })
            .collect::<Vec<(Coordinate, Option<T>)>>()
            .into_iter()
    }
}

/// Create a grid from data - two vecs that represent the rows and columns that are Some(T)
/// (represents an empty element) or None (an empty coordinate).
impl<T> TryFrom<Vec<Vec<Option<T>>>> for Grid<T> {
    type Error = GridCreationError;

    fn try_from(value: Vec<Vec<Option<T>>>) -> Result<Self, Self::Error> {
        // check that all the inner vecs are the same length
        let first_row_len = value.first().unwrap().len();

        if !value.iter().all(|row| row.len() == first_row_len) {
            // this is the wrong type of error
            return Err(GridCreationError {});
        };

        let y_size = value.len();
        let x_size = value.first().unwrap().iter().count();
        let mut grid_data: Vec<GridCoordinate<T>> = Vec::new();
        let mut result = Grid::new(x_size, y_size);
        for (y_count, line) in value.into_iter().enumerate() {
            for (x_count, element) in line.into_iter().enumerate() {
                let coordinate = result.to_grid_like([x_count, y_count]).unwrap();

                let grid_element = if let Some(val) = element {
                    GridCoordinate::Object::<T>(val)
                } else {
                    GridCoordinate::Empty::<T>(coordinate)
                };
                grid_data.push(grid_element);
            }
        }
        result.grid_data = grid_data;
        Ok(result)
    }
}

impl<T> OriginCenteredness for Grid<T> {
    type Distinguisher = OriginCentered;
}

impl<T> OriginBounded for Grid<T> {
    fn x_count(&self) -> usize {
        self.bounds.x_count()
    }

    fn y_count(&self) -> usize {
        self.bounds.y_count()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::AbsoluteDirection;
    use crate::BoundedMovingObject;
    use crate::Coordinate;
    use itertools::Itertools;
    use std::collections::HashMap;
    use std::fs::read_to_string;

    /// Checks that the boundaries of the grid are centered around the origin.
    fn assert_centered_around_origin<T>(input: &Grid<T>) {
        assert!(
            -input.x_min_boundary() == input.x_max_boundary()
                || input.x_max_boundary() == -input.x_min_boundary() + 1
        );
        assert!(
            -input.y_min_boundary() == input.y_max_boundary()
                || input.y_max_boundary() == -input.y_min_boundary() + 1
        );
    }

    #[allow(unused)]
    enum StoreValidity {
        Valid,
        Collision,
        OutOfBounds,
    }

    /// Checks that element doesn't panic for any coordinate in bounds.
    fn assert_coordinate_coverage<T>(input: &Grid<T>) {
        for (x, y) in (input.x_min_boundary()..=input.x_max_boundary())
            .cartesian_product(input.y_min_boundary()..=input.y_max_boundary())
        {
            input.element_unchecked(&Coordinate { x, y });
        }
    }

    /// Checks that the grid_data vec is consistent with the bounds in the struct. The bounds
    /// imply a length and the grid_data should be that length.
    fn assert_grid_data_and_bounds_consistency<T>(input: &Grid<T>) {
        let expected_count_by_bounds = input.bounds.x_count() * input.bounds.y_count();
        let actual_length = input.grid_data.len();
        assert_eq!(expected_count_by_bounds, actual_length);
    }

    fn check_grid_counts<T>(grid: &Grid<T>, x: usize, y: usize) {
        assert_eq!(grid.x_count(), x);
        assert_eq!(grid.y_count(), y)
    }

    fn empty_grid<T>(size: usize) -> Grid<T> {
        Grid::<T>::new(size, size)
    }

    fn rectangular_empty_grid(x_count: usize, y_count: usize) -> Grid<()> {
        Grid::new(x_count, y_count)
    }

    fn grid_with_occupied_corners_and_origin<T: Copy>(size: usize, element: T) -> Grid<T> {
        assert!(size > 0);
        let mut grid: Grid<T> = Grid::new(size, size);
        let (nw, sw, ne, se) = (
            grid.northwest_corner(),
            grid.southwest_corner(),
            grid.northeast_corner(),
            grid.southeast_corner(),
        );
        let origin = Coordinate::default();
        check_store(&mut grid, origin, element, StoreValidity::Valid);
        if size > 1 {
            check_store(&mut grid, nw, element, StoreValidity::Valid);
            if size > 2 {
                // should be origin if size <= 2
                check_store(&mut grid, sw, element, StoreValidity::Valid);
            }
            check_store(&mut grid, ne, element, StoreValidity::Valid);
            check_store(&mut grid, se, element, StoreValidity::Valid);
        }
        grid
    }

    // create a grid occupied with the elements at the occupied coordinates
    fn grid_with_occupied_at<T, const M: usize>(
        n: usize,
        occupied_coordinates: [Coordinate; M],
        elements: [T; M],
    ) -> Grid<T> {
        let mut grid = Grid::new(n, n);
        for (coordinate, element) in occupied_coordinates.iter().zip(elements) {
            check_store(&mut grid, *coordinate, element, StoreValidity::Valid)
        }
        grid
    }

    fn grid_with_single_element<T: Default>(size: usize, coordinate: Coordinate) -> Grid<T> {
        let mut grid: Grid<T> = Grid::new(size, size);
        check_store(&mut grid, coordinate, T::default(), StoreValidity::Valid);
        grid
    }

    #[track_caller]
    fn check_element<T: PartialEq + std::fmt::Debug>(
        grid: &Grid<T>,
        c: Coordinate,
        expected_element: &T,
    ) {
        if let Some(actual_element) = grid.element_unchecked(&c) {
            assert_eq!(actual_element, expected_element);
        } else {
            panic!();
        }

        if let Ok(actual_element) = grid.element(&c) {
            assert_eq!(actual_element, expected_element);
        } else {
            panic!();
        }
    }

    #[track_caller]
    fn check_empty<T: PartialEq + std::fmt::Debug>(grid: &Grid<T>, c: Coordinate) {
        assert!(grid.element_unchecked(&c).is_none())
    }

    #[track_caller]
    fn check_elements<const N: usize, T: PartialEq + std::fmt::Debug>(
        grid: &Grid<T>,
        coordinates: [Coordinate; N],
        elements: [&T; N],
    ) {
        for (coordinate, element) in coordinates.iter().zip(elements) {
            check_element(grid, *coordinate, element);
        }
    }

    #[track_caller]
    fn check_out_of_bounds<T: std::panic::RefUnwindSafe>(grid: &Grid<T>, c: Coordinate) {
        let result = std::panic::catch_unwind(|| {
            grid.element_unchecked(&c);
        });
        assert!(result.is_err())
    }

    #[track_caller]
    fn check_valid_remove<T: PartialEq + std::fmt::Debug>(
        grid: &mut Grid<T>,
        c: Coordinate,
        expected: T,
    ) {
        let res = grid.remove_element(&c);
        if let Ok(actual) = res {
            assert_eq!(expected, actual);
        } else {
            panic!();
        }
    }

    #[track_caller]
    fn check_store<T>(grid: &mut Grid<T>, c: Coordinate, element: T, is_valid: StoreValidity) {
        match is_valid {
            StoreValidity::OutOfBounds => {
                assert!(grid.store_element(&c, element).is_err())
            }

            StoreValidity::Collision => {
                if let Ok(previous_value) = grid.store_element(&c, element) {
                    assert!(previous_value.is_some());
                }
            }

            StoreValidity::Valid => {
                if let Ok(previous_value) = grid.store_element(&c, element) {
                    assert!(previous_value.is_none());
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum ExpectedMoveResponse {
        Valid,
        Collision,
        OutOfBounds,
    }

    fn check_move<T>(
        grid: &mut Grid<T>,
        c: Coordinate,
        direction: AbsoluteDirection,
        expected: ExpectedMoveResponse,
    ) {
        let res = grid.move_element_in_direction(&c, direction);

        match (expected, res.clone()) {
            (ExpectedMoveResponse::Valid, Ok(_)) => {}
            (ExpectedMoveResponse::Collision, Err(GridError::CollisionError)) => {}
            (
                ExpectedMoveResponse::OutOfBounds,
                Err(GridError::OutOfBoundsError(out_of_bounds_error)),
            ) => {
                assert_eq!(
                    out_of_bounds_error.position(),
                    c.coordinate_in_direction(direction, 1)
                );
            }
            _ => {
                println!("unexpected result in move:");
                println!("expected {expected:?}");
                println!("got {res:?}");
                panic!();
            }
        }
    }

    fn corners<T>(grid: &Grid<T>) -> [Coordinate; 4] {
        [
            grid.northwest_corner(),
            grid.southwest_corner(),
            grid.northeast_corner(),
            grid.southeast_corner(),
        ]
    }

    pub mod test_get {
        use super::*;

        #[test]
        pub fn single_element() {
            let grid = grid_with_occupied_at(3, [Coordinate::default()], [1]);
            assert_centered_around_origin(&grid);
            check_element(&grid, Coordinate::default(), &1);
        }

        #[test]
        pub fn single_element_two() {
            let grid = grid_with_occupied_at(4, [Coordinate::default()], [0]);
            assert_centered_around_origin(&grid);
            check_element(&grid, Coordinate::default(), &0);
        }

        #[test]
        pub fn corners_and_origin() {
            let grid = grid_with_occupied_corners_and_origin(5, 1);
            assert_centered_around_origin(&grid);
            check_elements(&grid, corners(&grid), [&1; 4]);
            check_element(&grid, Coordinate::default(), &1);
        }

        #[test]
        pub fn corners_and_origin_two() {
            let grid = grid_with_occupied_corners_and_origin(20, 1);
            assert_centered_around_origin(&grid);
            check_elements(&grid, corners(&grid), [&1; 4]);
            check_element(&grid, Coordinate::default(), &1);
        }

        #[test]
        pub fn out_of_bounds() {
            let grid: Grid<()> = empty_grid(5);
            check_out_of_bounds(&grid, Coordinate { x: 10, y: 10 });
        }

        #[test]
        pub fn out_of_bounds_two() {
            let grid: Grid<()> = empty_grid(0);
            check_out_of_bounds(&grid, Coordinate { x: -1, y: -1 });
        }
    }

    mod test_remove {
        use super::*;

        #[test]
        pub fn basic_remove() {
            let mut grid = grid_with_occupied_corners_and_origin(4, 1);
            check_valid_remove(&mut grid, Coordinate::default(), 1);
        }
    }

    #[test]
    pub fn should_move_element() {
        let mut grid: Grid<usize> = Grid::new(3, 3);
        let coordinate = Coordinate::default();
        let val: usize = 1;

        check_store(&mut grid, coordinate, val, StoreValidity::Valid);
        check_move(
            &mut grid,
            coordinate,
            AbsoluteDirection::North,
            ExpectedMoveResponse::Valid,
        );

        check_element(&grid, Coordinate { y: 1, x: 0 }, &1);
        check_empty(&grid, Coordinate::default());
    }

    #[test]
    pub fn move_should_collide() {
        let mut grid = grid_with_occupied_corners_and_origin(3, 1);
        check_move(
            &mut grid,
            Coordinate { x: 1, y: 1 },
            AbsoluteDirection::South,
            ExpectedMoveResponse::Valid,
        );
        check_move(
            &mut grid,
            Coordinate { x: 1, y: 0 },
            AbsoluteDirection::South,
            ExpectedMoveResponse::Collision,
        );
    }

    #[test]
    pub fn move_should_collide_two() {
        let mut grid = grid_with_occupied_corners_and_origin(2, 1);
        check_move(
            &mut grid,
            Coordinate { x: 1, y: 1 },
            AbsoluteDirection::South,
            ExpectedMoveResponse::Collision,
        );
    }

    #[test]
    pub fn move_should_be_out_of_bounds() {
        let mut grid = grid_with_occupied_corners_and_origin(1, 1);
        check_move(
            &mut grid,
            Coordinate { x: 0, y: 0 },
            AbsoluteDirection::South,
            ExpectedMoveResponse::OutOfBounds,
        );
    }

    #[test]
    fn test_boundaries() {
        for i in 1..=100 {
            let len = i;
            let grid: Grid<usize> = Grid::new(len, len);
            assert_eq!(
                grid.y_max_boundary() - grid.y_min_boundary(),
                (len - 1) as i32
            );
            assert_eq!(
                grid.x_max_boundary() - grid.x_min_boundary(),
                (len - 1) as i32
            );
            assert_centered_around_origin(&grid);
        }
    }

    #[test]
    fn coordinate_to_index() {
        for n in 1..100 {
            let grid: Grid<()> = Grid::new(n, n);
            assert_eq!(
                grid.coordinate_to_index(&grid.northwest_corner()).unwrap(),
                0
            );
            assert_eq!(
                grid.coordinate_to_index(&grid.northeast_corner()).unwrap(),
                n - 1
            );
            assert_eq!(
                grid.coordinate_to_index(&grid.southwest_corner()).unwrap(),
                grid.grid_data.len() - n
            );
            assert_eq!(
                grid.coordinate_to_index(&grid.southeast_corner()).unwrap(),
                grid.grid_data.len() - 1
            );
        }
    }

    mod test_status_string {

        use super::*;

        fn check_string<T>(grid: &Grid<T>, expected: &str) {
            assert_eq!(grid.element_statuses(), expected.to_string())
        }

        #[test]
        fn empty_one_by_one() {
            let grid: Grid<()> = Grid::new(1, 1);
            check_string(&grid, ".");
        }

        #[test]
        fn occupied_one_by_one() {
            let mut grid: Grid<()> = Grid::new(1, 1);
            let _ = grid.store_element(&Coordinate::default(), ());
            check_string(&grid, "#");
        }

        #[test]
        fn partially_occupied_one_by_two() {
            let mut grid: Grid<()> = Grid::new(1, 2);
            let _ = grid.store_element(&Coordinate::default(), ());
            check_string(&grid, ".\n#");
        }

        #[test]
        fn empty_two_by_two() {
            let grid: Grid<()> = Grid::new(2, 2);
            check_string(&grid, "..\n..");
        }

        #[test]
        fn corner_occupied_three_by_three() {
            let mut grid: Grid<()> = Grid::new(3, 3);
            let _ = grid.store_element(&grid.northwest_corner(), ());
            let _ = grid.store_element(&grid.northeast_corner(), ());
            let _ = grid.store_element(&grid.southwest_corner(), ());
            let _ = grid.store_element(&grid.southeast_corner(), ());
            check_string(&grid, "#.#\n...\n#.#");
        }
    }

    mod test_bounded_neighbors_to {

        use super::*;

        #[track_caller]
        fn check_bounded_neighbors_to_count<T>(
            grid: &Grid<T>,
            coordinate: impl Positioned,
            count: usize,
        ) {
            let iter = grid.bounded_neighbors_to(coordinate);
            assert_eq!(iter.count(), count);
        }

        #[track_caller]
        fn check_contains_corner<T>(grid: &Grid<T>, coordinate: impl Positioned) {
            assert!(
                grid.bounded_neighbors_to(coordinate.position())
                    .contains(&coordinate.coordinate_in_direction(AbsoluteDirection::North, 1))
            );
            assert!(
                grid.bounded_neighbors_to(coordinate.position())
                    .contains(&coordinate.coordinate_in_direction(AbsoluteDirection::South, 1))
            );
            assert!(
                grid.bounded_neighbors_to(coordinate.position())
                    .contains(&coordinate.coordinate_in_direction(AbsoluteDirection::East, 1))
            );
            assert!(
                grid.bounded_neighbors_to(coordinate.position())
                    .contains(&coordinate.coordinate_in_direction(AbsoluteDirection::West, 1))
            )
        }

        #[test]
        fn test_origin_only_grid() {
            let grid: Grid<()> = empty_grid(1);
            check_bounded_neighbors_to_count(&grid, Coordinate::default(), 0);
        }

        #[test]
        #[should_panic]
        fn test_origin_only_should_not_contain_corners() {
            let grid: Grid<()> = empty_grid(1);
            check_contains_corner(&grid, Coordinate::default());
        }

        #[test]
        #[should_panic]
        fn test_two_by_two_should_not_contain_corners() {
            let grid: Grid<()> = empty_grid(2);
            check_contains_corner(&grid, Coordinate::default());
        }

        #[test]
        fn test_three_by_three() {
            let grid: Grid<()> = empty_grid(3);
            check_bounded_neighbors_to_count(&grid, Coordinate::default(), 8);
            check_contains_corner(&grid, Coordinate::default());
        }

        #[test]
        fn test_five_by_five() {
            let grid: Grid<()> = empty_grid(5);
            check_bounded_neighbors_to_count(&grid, Coordinate::default(), 8);
        }

        #[test]
        fn test_three_by_one() {
            let grid: Grid<()> = rectangular_empty_grid(3, 1);
            check_bounded_neighbors_to_count(&grid, Coordinate::default(), 2);
        }

        #[test]
        fn test_two_by_one() {
            let grid: Grid<()> = rectangular_empty_grid(2, 1);
            check_bounded_neighbors_to_count(&grid, Coordinate::default(), 1);
        }

        #[test]
        fn empty_iterator_on_point_outside_bounds() {
            let grid: Grid<()> = empty_grid(2);
            check_bounded_neighbors_to_count(&grid, Coordinate { x: 5, y: 0 }, 0);
        }
    }

    pub mod iterator_tests {
        use super::*;

        #[test]
        fn iter_new_len() {
            for i in 1..100 {
                let grid: Grid<()> = Grid::new(i, i);
                assert!(
                    grid.iter_new()
                        .map(|(_, element)| element)
                        .all(|x| x.is_none())
                );
                assert_eq!(grid.iter_new().count(), i * i);
            }
        }

        #[test]
        fn iter_mut_new_by_increasing_elements() {
            let n = 11;
            let mut grid = grid_with_occupied_corners_and_origin(n, 1);
            let corners = corners(&grid);
            let origin = Coordinate::default();

            for (coordinate, element) in grid.iter_mut_new() {
                if corners.contains(&coordinate) || coordinate == origin {
                    if let Some(value) = element {
                        *value += 1;
                    }
                }
            }

            for (coordinate, element) in grid.iter_mut_new() {
                if corners.contains(&coordinate) || coordinate == origin {
                    assert_eq!(element, Some(&mut 2));
                } else {
                    assert_eq!(element, None)
                }
            }
        }

        #[test]
        fn iter_new_with_element() {
            let n = 11;
            let origin = Coordinate::default();
            let grid = grid_with_occupied_corners_and_origin(n, 1);
            let corners = corners(&grid);
            for (coordinate, element) in grid.iter_new() {
                if coordinate == origin || corners.contains(&coordinate) {
                    assert_eq!(element, Some(&1));
                } else {
                    assert_eq!(element, None);
                }
            }
        }

        #[test]
        fn iter_elements_new_count() {
            let grid = grid_with_occupied_corners_and_origin(100, 1);
            assert_eq!(grid.iter_elements_new().count(), 5);
        }

        #[test]
        fn iter_mut_elements_new_count() {
            let mut grid = grid_with_occupied_corners_and_origin(100, 1);
            assert_eq!(grid.iter_mut_elements_new().count(), 5);
        }

        #[test]
        fn iter_mut_count() {
            for i in 1..=100 {
                let len = i;
                let mut grid: Grid<()> = Grid::new(len, len);
                assert_eq!(grid.iter_mut_new().count(), len * len);
                assert_eq!(grid.iter_new().count(), len * len);
            }
        }
    }

    fn hashtag_occupied_map() -> HashMap<char, ()> {
        let mut map = HashMap::new();
        map.insert('.', ());
        map
    }

    fn symmetric_shape_should_transpose_to_itself(path: &str) {
        let map = hashtag_occupied_map();
        let original_grid: Grid<()> =
            Grid::from_str_by_map(&read_to_string(path).unwrap(), &map).unwrap();

        let mut changed_grid: Grid<()> =
            Grid::from_str_by_map(&read_to_string(path).unwrap(), &map).unwrap();

        changed_grid.transpose_new();
        assert_eq!(original_grid, changed_grid);
    }

    #[test]
    fn coordinates_in_direction() {
        let len = 5;
        let grid: Grid<()> = Grid::new(len, len);
        let marker = BoundedMovingObject::try_from((&grid, Coordinate::default())).unwrap();
        let result = marker.coordinates_in_direction(AbsoluteDirection::South);
        assert!(result.contains(&Coordinate { y: -1, x: 0 }));
        assert!(result.contains(&Coordinate { y: -2, x: 0 }));
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn coordinate_to_index_and_index_to_coordinate_inverse_functions() {
        let grid: Grid<Coordinate> = Grid::new(100, 100);
        for (grid_coordinate, _) in grid.iter_new() {
            println!(
                "check index for coordinate {:?}",
                grid_coordinate.position()
            );
            let index = grid.coordinate_to_index(&grid_coordinate).unwrap();
            println!("index is {index}");
            let coordinate = grid.index_to_coordinate(index).unwrap();
            assert_eq!(grid_coordinate.position(), &coordinate);
        }
    }

    #[test]
    fn neighbor_in_direction_from() {
        use AbsoluteDirection::*;
        let grid: Grid<Coordinate> = Grid::new(20, 20);

        let directions = [North, East, South, West];
        for (coord, _) in grid.iter_new() {
            for direction in directions.iter() {
                if let Some(neighbor) = grid.neighbor_in_direction_from(&coord, *direction) {
                    assert_eq!(coord.manhattan_distance_to(&neighbor), 1);
                    assert_eq!(coord.direction_toward(neighbor.position()).0, *direction);
                    assert_eq!(coord.direction_toward(neighbor.position()).1, *direction);
                } else {
                    assert!(grid.other_is_on_border(&coord));
                }
            }
        }
    }

    pub mod constructor_tests {
        use super::*;

        #[test]
        fn new_from_str_test() {
            let input = "...";
            let map: HashMap<char, usize> = HashMap::new();
            let data = Grid::<usize>::from_str_by_map(input, &map).unwrap();
            check_grid_counts(&data, 3, 1);
            assert_eq!(data.iter_elements_new().count(), 0);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }

        #[test]
        fn test_new_from_str_unwrapped_with_empty_with_deprecated_fns() {
            let input = "...";
            let map: HashMap<char, usize> = HashMap::new();
            let data = Grid::<usize>::from_str_by_map(input, &map).unwrap();
            check_grid_counts(&data, 3, 1);
            assert_eq!(data.iter_elements_new().count(), 0);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }

        #[test]
        #[should_panic]
        fn new_from_str_unwrapped_should_panic_when_rows_are_different_sizes() {
            let input = "...\n....";
            let map: HashMap<char, usize> = HashMap::new();
            Grid::<usize>::from_str_by_map(input, &map).unwrap();
        }
    }

    pub mod transpose_tests {

        use super::*;
        use std::fs::read_to_string;

        #[test]
        fn test_transpose() {
            let mut grid: Grid<usize> = Grid::new(3, 1);
            grid.transpose_new();
            check_grid_counts(&grid, 1, 3);

            let mut grid: Grid<usize> = Grid::new(1, 1);
            grid.transpose_new();
            check_grid_counts(&grid, 1, 1);

            let mut grid: Grid<usize> = Grid::new(3, 3);
            check_store(
                &mut grid,
                Coordinate { x: -1, y: -1 },
                1,
                StoreValidity::Valid,
            );

            grid.transpose_new();
            check_element(&grid, Coordinate { x: 1, y: 1 }, &1);
            check_empty(&grid, Coordinate { x: -1, y: -1 });
            assert_centered_around_origin(&grid);
        }

        #[test]
        fn transpose_new_trivial() {
            let n = 3;
            let element = 1;
            let mut grid: Grid<usize> = Grid::new(n, n);
            let [_, ne, sw, _] = corners(&grid);
            check_store(&mut grid, ne, element, StoreValidity::Valid);
            grid.transpose_new();
            check_element(&grid, sw, &1);
            check_empty(&grid, ne);
        }

        /// Testing that double transpose yields original grid.
        #[test]
        fn double_transpose_test() {
            let input_data = read_to_string("tests/data/row_expansion_test_1.txt").unwrap();
            let mut map: HashMap<char, ()> = HashMap::new();

            map.insert('#', ());

            let mut grid: Grid<()> = Grid::from_str_by_map(&input_data, &map).unwrap();
            let expected_result_grid: Grid<()> = Grid::from_str_by_map(&input_data, &map).unwrap();

            grid.transpose_new();

            assert_coordinate_coverage(&grid);
            assert_centered_around_origin(&grid);
            assert_grid_data_and_bounds_consistency(&grid);
            grid.transpose_new();
            assert_coordinate_coverage(&grid);
            assert_centered_around_origin(&grid);

            assert_eq!(grid, expected_result_grid);
        }

        #[test]
        fn double_transpose_test_two() {
            let input_data = read_to_string("tests/data/row_expansion_test_3.txt").unwrap();
            let mut map: HashMap<char, ()> = HashMap::new();
            map.insert('#', ());

            let mut grid: Grid<()> = Grid::from_str_by_map(&input_data, &map).unwrap();
            let expected_result_grid: Grid<()> = Grid::from_str_by_map(&input_data, &map).unwrap();

            grid.transpose_new();
            assert_coordinate_coverage(&grid);
            assert_centered_around_origin(&grid);
            grid.transpose_new();
            assert_coordinate_coverage(&grid);
            assert_centered_around_origin(&grid);
            assert_eq!(grid, expected_result_grid);
        }

        #[test]
        fn double_transpose_test_three() {
            let input_data =
                read_to_string("tests/data/row_expansion_test_3_expected_result.txt").unwrap();
            let mut map: HashMap<char, ()> = HashMap::new();
            map.insert('#', ());

            let mut grid: Grid<()> = Grid::from_str_by_map(&input_data, &map).unwrap();
            let expected_result_grid: Grid<()> = Grid::from_str_by_map(&input_data, &map).unwrap();

            grid.transpose_new();
            grid.transpose_new();
            assert_eq!(grid, expected_result_grid);
        }

        #[test]
        pub fn edges_only_should_tranpose_to_itself() {
            symmetric_shape_should_transpose_to_itself("tests/data/edges_only.txt")
        }

        #[test]
        pub fn cross_should_transpose_to_itself() {
            symmetric_shape_should_transpose_to_itself("tests/data/cross.txt");
        }
    }

    pub mod row_expansion {

        use super::*;

        #[test]
        fn add_row_test() {
            let mut grid: Grid<Coordinate> = Grid::new(3, 3);
            for _ in 1..10 {
                grid.add_row();
                assert_coordinate_coverage(&grid);
                assert_centered_around_origin(&grid);
            }
        }

        #[test]
        fn row_expansion_test() {
            let old_len = 3;
            let mut grid: Grid<Coordinate> = Grid::new(old_len, old_len);
            assert_eq!(grid.y_count(), 3);
            assert_coordinate_coverage(&grid);
            assert_centered_around_origin(&grid);
            grid.add_row();
            assert_coordinate_coverage(&grid);
            assert_centered_around_origin(&grid);
            assert_eq!(grid.y_count(), 4);
        }

        #[test]
        fn basic_upward_row_expansion_above_element() {
            let c = Coordinate { x: 0, y: 0 };
            let mut grid: Grid<usize> = grid_with_single_element(3, c);

            assert_eq!(grid.y_count(), 3);
            grid.expand_at_row(1);
            assert_eq!(grid.y_count(), 4);

            // expansion happens upwards above the element so the object should not move.
            check_element(&grid, c, &0);
        }

        #[test]
        fn basic_upward_row_expansion_below_element() {
            let c = Coordinate { x: 0, y: 0 };
            let mut grid: Grid<usize> = grid_with_single_element(3, c);
            grid.expand_at_row(-1);

            // expansion happens upwards below the element so the object should move.
            check_empty(&grid, c);
            check_element(&grid, Coordinate { y: 1, x: 0 }, &0);
        }

        #[test]
        fn basic_upward_row_expansion_on_element_row() {
            let c = Coordinate { x: 0, y: 0 };
            let mut grid: Grid<usize> = grid_with_single_element(3, c);
            grid.expand_at_row(0);

            // expansion happens on the element row so the object should move upwards.
            check_empty(&grid, c);
            check_element(&grid, Coordinate { y: 1, x: 0 }, &0);
        }

        #[test]
        fn basic_downward_row_expansion_below_element() {
            let c = Coordinate { x: 0, y: 0 };
            let mut grid: Grid<usize> = grid_with_single_element(4, c);
            grid.expand_at_row(-1);
            check_element(&grid, c, &0);
        }

        #[test]
        fn basic_downward_row_expansion_above_element() {
            let c = Coordinate { x: 0, y: 0 };
            let mut grid: Grid<usize> = grid_with_single_element(4, c);
            grid.expand_at_row(1);
            check_empty(&grid, c);
        }

        #[test]
        fn basic_downward_row_expansion_on_element_row() {
            let c = Coordinate { x: 0, y: 0 };
            let mut grid: Grid<usize> = grid_with_single_element(4, c);
            grid.expand_at_row(0);
            check_empty(&grid, c);
        }
    }
}
