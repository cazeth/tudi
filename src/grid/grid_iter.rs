use super::Grid;
use crate::AbsoluteDirection;
use crate::BoundedMovingObject;
use crate::Coordinate;
use crate::Positioned;
use crate::bounded::Bounded;

pub struct GridIter<'a, T> {
    current: BoundedMovingObject,
    grid: &'a Grid<T>,
    visited_last: bool,
}

impl<'a, T> GridIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        let mut current = BoundedMovingObject::from_bounded(&grid.bounds());
        current.set_current_x_to_x_min();
        current.set_current_y_to_y_max();
        Self {
            current,
            grid,
            visited_last: false,
        }
    }
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Coordinate, Option<&'a T>);
    fn next(&mut self) -> Option<Self::Item> {
        if &self.current.southeast_corner() == self.current.position() {
            if self.visited_last {
                return None;
            } else {
                self.visited_last = true;
            }
        }

        let result = (
            *self.current.position(),
            self.grid.element_unchecked(self.current.position()),
        );

        if self
            .current
            .move_in_absolute_direction(AbsoluteDirection::East, 1)
        {
        } else if self
            .current
            .move_in_absolute_direction(AbsoluteDirection::South, 1)
        {
            self.current.set_current_x_to_x_min();
        };

        Some(result)
    }
}
