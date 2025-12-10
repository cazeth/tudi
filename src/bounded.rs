use crate::AbsoluteDirection;
use crate::Coordinate;
use crate::Mover;
use crate::OutOfBoundsError;
use crate::Positioned;

#[allow(private_bounds)]
pub trait Bounded: BoundSeal {
    fn x_min_boundary(&self) -> i32;

    fn x_max_boundary(&self) -> i32;

    fn y_min_boundary(&self) -> i32;

    fn y_max_boundary(&self) -> i32;

    fn southeast_corner(&self) -> Coordinate {
        Coordinate {
            x: self.x_max_boundary(),
            y: self.y_min_boundary(),
        }
    }

    fn southwest_corner(&self) -> Coordinate {
        Coordinate {
            x: self.x_min_boundary(),
            y: self.y_min_boundary(),
        }
    }

    fn northwest_corner(&self) -> Coordinate {
        Coordinate {
            x: self.x_min_boundary(),
            y: self.y_max_boundary(),
        }
    }

    fn northeast_corner(&self) -> Coordinate {
        Coordinate {
            x: self.x_max_boundary(),
            y: self.y_max_boundary(),
        }
    }

    fn x_count(&self) -> usize {
        (self.x_max_boundary() - self.x_min_boundary()).unsigned_abs() as usize + 1
    }

    fn y_count(&self) -> usize {
        (self.y_max_boundary() - self.y_min_boundary()).unsigned_abs() as usize + 1
    }

    /// Returns the euclidian length between the extremes of the bound.
    /// ```
    /// use tudi::Grid;
    /// use tudi::Bounded;
    /// let grid: Grid<()> = Grid::new(3,3); // a three by three grid.
    /// assert_eq!(grid.x_geometric_len(), 2); // the distance between -1 and 1 is 2
    ///
    /// ```
    fn x_geometric_len(&self) -> usize {
        (self.x_max_boundary() - self.x_min_boundary()).unsigned_abs() as usize
    }

    /// Returns the euclidian length between the extremes of the bound.
    /// ```
    /// use tudi::Grid;
    /// use tudi::Bounded;
    /// let grid: Grid<()> = Grid::new(3, 3); // a three by three grid.
    /// assert_eq!(grid.y_geometric_len(), 2); // the distance between -1 and 1 is 2
    ///
    /// ```
    fn y_geometric_len(&self) -> usize {
        (self.y_max_boundary() - self.y_min_boundary()).unsigned_abs() as usize
    }

    fn is_within_bounds<T: Positioned>(&self, coordinate: &T) -> bool {
        self.x_min_boundary() <= coordinate.x_coordinate()
            && self.x_max_boundary() >= coordinate.x_coordinate()
            && self.y_min_boundary() <= coordinate.y_coordinate()
            && self.y_max_boundary() >= coordinate.y_coordinate()
    }

    ///Checks if the an external GridObject is on the border of the bounded region. The function
    ///panics if the coordinate is out of bounds.
    fn other_is_on_border<C: Positioned>(&self, coordinate: &C) -> bool {
        coordinate.x_coordinate() == self.x_min_boundary()
            || coordinate.x_coordinate() == self.x_max_boundary()
            || coordinate.y_coordinate() == self.y_max_boundary()
            || coordinate.y_coordinate() == self.y_min_boundary()
    }

    fn coordinates_in_direction_from<C: Positioned>(
        &self,
        coordinate: &C,
        direction: AbsoluteDirection,
    ) -> Vec<Coordinate>
    where
        Self: Sized,
    {
        let mut coordinate = *coordinate.position();
        let mut result = Vec::new();

        loop {
            coordinate = coordinate.coordinate_in_direction(direction, 1);
            if self.is_within_bounds(&coordinate) {
                result.push(coordinate);
            } else {
                break;
            }
        }

        result
    }

    /// Returns the index of a coordinate in a region if it were counted from west to east, north to
    /// south.
    /// # Examples
    /// ```
    /// use tudi::Grid;
    /// use tudi::Coordinate;
    /// use tudi::Bounded;
    /// let n = 5;
    /// let grid : Grid<Coordinate> = Grid::new(n, n);
    /// assert_eq!( grid.coordinate_to_index(&grid.northwest_corner()).unwrap(), 0);
    /// assert_eq!( grid.coordinate_to_index(&grid.southeast_corner()).unwrap(), n*n-1);
    /// ```
    fn coordinate_to_index<C: Positioned>(
        &self,
        coordinate: &C,
    ) -> Result<usize, OutOfBoundsError> {
        if !self.is_within_bounds(coordinate) {
            Err(OutOfBoundsError::new(*coordinate.position()))
        } else {
            let [x_matrix_like, y_matrix_like] = self.to_matrix_like(coordinate.position());
            Ok(y_matrix_like * self.x_count() + x_matrix_like)
        }
    }

    /// Returns the coordinate at an index, counted(started at zero) , counted by row from the
    /// northwest corner by row.
    /// panics if index is out of bounds
    fn index_to_coordinate(&self, index: usize) -> Result<Coordinate, OutOfBoundsError> {
        let y_matrix_like = index / self.x_count();
        let x_matrix_like = index - y_matrix_like * self.x_count();
        self.to_grid_like([x_matrix_like, y_matrix_like])
    }

    /// returns true if the object is currently on its border.
    fn is_on_border(&self) -> bool
    where
        Self: Positioned,
    {
        self.position().x == self.x_min_boundary()
            || self.position().x == self.x_max_boundary()
            || self.position().y == self.y_max_boundary()
            || self.position().y == self.y_min_boundary()
    }

    /// Returns the diff coordinates from the northwest corner to a coordinate. This can be thought of as mapping a coordinate
    /// symmetric around the origin (like x_min = -2, x_max = 2) to matrix-like (x_min
    /// = 0, x_max = 4)
    /// 0,0 means the northwest corner, as with standard matrix indices.
    /// panics if the coordinate is out of bounds.
    /// # Examples
    /// ```
    /// use tudi::Grid;
    /// use tudi::Coordinate;
    /// use tudi::Bounded;
    /// let grid : Grid<Coordinate> = Grid::new(3, 3);
    /// let origin = Coordinate{x: 0, y : 0};
    /// assert_eq!(grid.to_matrix_like(&origin), [1,1]);
    /// ```
    fn to_matrix_like<C: Positioned>(&self, coord: &C) -> [usize; 2] {
        assert!(self.is_within_bounds(coord));
        [
            coord.x_coordinate().abs_diff(self.x_min_boundary()) as usize,
            self.y_max_boundary().abs_diff(coord.y_coordinate()) as usize,
        ]
    }

    /// inverse of count from min: Given a distance from the northwest corner, the function returns the coordinates.
    /// array is of form \[x,y\]
    /// panics if the coordinate is out of bounds.
    fn to_grid_like(&self, distance: [usize; 2]) -> Result<Coordinate, OutOfBoundsError> {
        let coordinate = Coordinate {
            x: self.x_min_boundary() + distance[0] as i32,
            y: self.y_max_boundary() - distance[1] as i32,
        };

        if self.is_within_bounds(&coordinate) {
            Ok(coordinate)
        } else {
            Err(OutOfBoundsError::new(coordinate))
        }
    }

    /// Returns the nearest neighbor to a position in a given direction. If the neighbor in that
    /// direction is out of bounds, the function returns None.
    fn neighbor_in_direction_from<C: Positioned>(
        &self,
        position: &C,
        direction: AbsoluteDirection,
    ) -> Option<Coordinate> {
        let potential_neighbor = position.coordinate_in_direction(direction, 1);
        if self.is_within_bounds(&potential_neighbor) {
            Some(potential_neighbor)
        } else {
            None
        }
    }

    /// Similar to [`Positioned::manhattan_neighbors`], this function returns the immediately adjacent
    /// coordinate to the current coordinate. It also considers boundaries and filters out
    /// coordinates that aren't within on or them.
    fn bounded_neighbors(&self) -> Vec<Coordinate>
    where
        Self: Positioned,
    {
        let candidate_coordinates = Positioned::euclid_neighbors(self.position());
        candidate_coordinates
            .into_iter()
            .filter(|x| self.is_within_bounds(x))
            .collect::<Vec<Coordinate>>()
    }

    /// Get the within-bounds euclid neighbors of a point.
    ///
    /// The iterator only returns coordinates that are within bounds. Thus an input coordinate on the border would typically yield an iterator with a smaller count than an element in the
    /// center.
    /// If the input point is outside the bounds, the iterator returns empty.
    /// See also [Bounded::bounded_neighbors]
    fn bounded_neighbors_to<C: Positioned>(
        &self,
        coordinate: C,
    ) -> impl Iterator<Item = Coordinate> {
        coordinate
            .euclid_neighbors()
            .into_iter()
            .filter(|x| self.is_within_bounds(x))
    }

    /// Returns true if the coordinate actually moved and false if not, if there is
    /// an attempt to move outside of the border.
    fn move_in_absolute_direction(&mut self, direction: AbsoluteDirection, magnitude: u32) -> bool
    where
        Self: Mover,
    {
        let previous_coordinate = *self.position();
        let new_potential_coordinate = self
            .position()
            .coordinate_in_direction(direction, magnitude);

        let x = std::cmp::max(
            std::cmp::min(new_potential_coordinate.x, self.x_max_boundary()),
            self.x_min_boundary(),
        );
        let y = std::cmp::max(
            std::cmp::min(new_potential_coordinate.y, self.y_max_boundary()),
            self.y_min_boundary(),
        );

        self.set_coordinate(&Coordinate { x, y });
        previous_coordinate != *self.position()
    }
}

pub trait OriginCenteredness {
    type Distinguisher;
}

impl<T: OriginCenteredness> OriginCenteredness for &T {
    type Distinguisher = T::Distinguisher;
}

impl<T: OriginCenteredness> OriginCenteredness for &mut T {
    type Distinguisher = T::Distinguisher;
}

pub trait OriginBounded: OriginCenteredness<Distinguisher = OriginCentered> {
    fn x_count(&self) -> usize;
    fn y_count(&self) -> usize;
}

impl<T: OriginBounded> OriginBounded for &T {
    fn y_count(&self) -> usize {
        T::y_count(self)
    }

    fn x_count(&self) -> usize {
        T::x_count(self)
    }
}

impl<T: OriginBounded> OriginBounded for &mut T {
    fn y_count(&self) -> usize {
        T::y_count(self)
    }

    fn x_count(&self) -> usize {
        T::x_count(self)
    }
}

pub trait MaybeOriginBounded: OriginCenteredness<Distinguisher = MaybeOriginCentered> {
    fn x_min(&self) -> i32;
    fn x_max(&self) -> i32;
    fn y_min(&self) -> i32;
    fn y_max(&self) -> i32;
}

impl<T: MaybeOriginBounded> MaybeOriginBounded for &T {
    fn x_min(&self) -> i32 {
        T::x_min(self)
    }
    fn x_max(&self) -> i32 {
        T::x_max(self)
    }
    fn y_min(&self) -> i32 {
        T::y_min(self)
    }

    fn y_max(&self) -> i32 {
        T::y_max(self)
    }
}

impl<T: MaybeOriginBounded> MaybeOriginBounded for &mut T {
    fn x_min(&self) -> i32 {
        T::x_min(self)
    }
    fn x_max(&self) -> i32 {
        T::x_max(self)
    }
    fn y_min(&self) -> i32 {
        T::y_min(self)
    }

    fn y_max(&self) -> i32 {
        T::y_max(self)
    }
}

//pub struct NotOriginCentered;
pub struct OriginCentered;
pub struct MaybeOriginCentered;

trait BoundsHelper<ObjType>: OriginCenteredness {
    fn x_min_boundary(&self) -> i32;
    fn x_max_boundary(&self) -> i32;
    fn y_min_boundary(&self) -> i32;
    fn y_max_boundary(&self) -> i32;
}

impl<T> BoundsHelper<OriginCentered> for T
where
    T: OriginBounded,
{
    fn x_min_boundary(&self) -> i32 {
        if self.x_count() % 2 == 0 {
            -(BoundsHelper::x_max_boundary(self) - 1)
        } else {
            -(BoundsHelper::x_max_boundary(self))
        }
    }

    fn x_max_boundary(&self) -> i32 {
        if self.x_count() == 0 {
            return 0;
        };

        self.x_count() as i32 / 2
    }

    fn y_min_boundary(&self) -> i32 {
        if self.y_count() % 2 == 0 {
            -(BoundsHelper::y_max_boundary(self) - 1)
        } else {
            -(BoundsHelper::y_max_boundary(self))
        }
    }

    fn y_max_boundary(&self) -> i32 {
        if self.y_count() == 0 {
            return 0;
        };

        self.y_count() as i32 / 2
    }
}

impl<T> BoundsHelper<MaybeOriginCentered> for T
where
    T: MaybeOriginBounded,
{
    fn x_min_boundary(&self) -> i32 {
        self.x_min()
    }

    fn x_max_boundary(&self) -> i32 {
        self.x_max()
    }

    fn y_min_boundary(&self) -> i32 {
        self.y_min()
    }

    fn y_max_boundary(&self) -> i32 {
        self.y_max()
    }
}

impl<T: BoundsHelper<<Self as OriginCenteredness>::Distinguisher>> BoundSeal for T {
    fn x_min_seal(&self) -> i32 {
        BoundsHelper::x_min_boundary(self)
    }

    fn x_max_seal(&self) -> i32 {
        BoundsHelper::x_max_boundary(self)
    }

    fn y_min_seal(&self) -> i32 {
        BoundsHelper::y_min_boundary(self)
    }

    fn y_max_seal(&self) -> i32 {
        BoundsHelper::y_max_boundary(self)
    }
}

trait BoundSeal {
    fn x_min_seal(&self) -> i32;
    fn y_min_seal(&self) -> i32;
    fn x_max_seal(&self) -> i32;
    fn y_max_seal(&self) -> i32;
}

pub trait OriginCenteredBounded {
    fn x_count(&self) -> u32;

    fn y_count(&self) -> u32;
}

impl<T: OriginCenteredBounded> OriginCenteredBounded for &T {
    fn x_count(&self) -> u32 {
        T::x_count(self)
    }

    fn y_count(&self) -> u32 {
        T::x_count(self)
    }
}

impl<T: BoundSeal> Bounded for T {
    fn x_min_boundary(&self) -> i32 {
        self.x_min_seal()
    }

    fn x_max_boundary(&self) -> i32 {
        self.x_max_seal()
    }
    fn y_min_boundary(&self) -> i32 {
        self.y_min_seal()
    }
    fn y_max_boundary(&self) -> i32 {
        self.y_max_seal()
    }
}
