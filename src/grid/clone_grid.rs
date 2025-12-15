use super::{Grid, GridCreationError};
use std::collections::HashMap;

impl<T: Clone> Grid<T> {
    /// Creates a grid from a str where each lines represents a row. Each character in the string
    /// is mapped to a grid element or an empty coordinate according to the provided hashmap.
    /// If a character is not in the map, the character is set to empty.
    ///
    /// The method creates a new cloned T at each occupied point in the Grid.
    ///
    /// # Panics
    /// This method panics if any rows in the input str are of different lengths.
    ///
    pub fn from_str_by_map(
        input: &str,
        map: &HashMap<char, T>,
    ) -> Result<Grid<T>, GridCreationError> {
        let mut char_data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let data = char_data
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|c| map.get(c).cloned())
                    .collect::<Vec<Option<T>>>()
            })
            .collect::<Vec<Vec<Option<T>>>>();

        Grid::<T>::try_from(data)
    }
}

impl<T: Clone> PartialEq for Grid<T> {
    /// This does not check T for equality (since T does not necessarily implement partialEq), but it checks
    /// that types, sizes and so forth are identical.
    fn eq(&self, other: &Self) -> bool {
        // check meta.
        if self.x_count() != other.x_count() || self.y_count() != other.y_count() {
            return false;
        };
        self.iter_new()
            .zip(other.iter_new())
            .all(|((_, a), (_, b))| a.is_some() == b.is_some())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::Coordinate;
    use crate::bounded::Bounded;
    use itertools::Itertools;

    /// Checks that the boundaries of the grid are centered around the origin.
    fn assert_centered_around_origin<T: Clone>(input: &Grid<T>) {
        assert!(
            -input.x_min_boundary() == input.x_max_boundary()
                || input.x_max_boundary() == -input.x_min_boundary() + 1
        );
        assert!(
            -input.y_min_boundary() == input.y_max_boundary()
                || input.y_max_boundary() == -input.y_min_boundary() + 1
        );
    }

    /// Checks that element doesn't panic for any coordinate in bounds.
    fn assert_coordinate_coverage<T: Clone>(input: &Grid<T>) {
        for (x, y) in (input.x_min_boundary()..=input.x_max_boundary())
            .cartesian_product(input.y_min_boundary()..=input.y_max_boundary())
        {
            input.element_unchecked(&Coordinate { x, y });
        }
    }

    #[track_caller]
    fn check_x_count<T>(grid: &Grid<T>, count: usize) {
        assert_eq!(grid.x_count(), count);
    }

    #[track_caller]
    fn check_y_count<T>(grid: &Grid<T>, count: usize) {
        assert_eq!(grid.y_count(), count);
    }

    pub mod constructor_tests {
        use super::*;

        #[test]
        fn new_from_str_test() {
            let input = "...";
            let map: HashMap<char, usize> = HashMap::new();
            let data = Grid::<usize>::from_str_by_map(input, &map).unwrap();
            check_x_count(&data, 3);
            check_y_count(&data, 1);
            assert_eq!(data.iter_elements_new().count(), 0);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }

        #[test]
        fn new_from_str_test_two() {
            let input = ".x.";
            let mut map: HashMap<char, usize> = HashMap::new();
            map.insert('x', 1);
            let data = Grid::<usize>::from_str_by_map(input, &map).unwrap();
            check_x_count(&data, 3);
            check_y_count(&data, 1);
            assert_eq!(data.iter_elements_new().count(), 1);
            assert_eq!(*data.element(&Coordinate::default()).unwrap(), 1);
        }

        #[test]
        fn test_new_from_str_unwrapped_with_empty() {
            let input = "...";
            let map: HashMap<char, usize> = HashMap::new();
            let data = Grid::<usize>::from_str_by_map(input, &map).unwrap();
            check_x_count(&data, 3);
            check_y_count(&data, 1);
            assert_eq!(data.iter_elements_new().count(), 0);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }

        #[test]
        fn new_from_str_unwrapped_should_panic_when_rows_are_different_sizes() {
            let input = "...\n....";
            let map: HashMap<char, usize> = HashMap::new();
            let res = Grid::<usize>::from_str_by_map(input, &map);
            assert!(res.is_err())
        }

        #[test]
        fn empty_rows_one() {
            let input = ".#.\n...";
            let mut map: HashMap<char, ()> = HashMap::new();
            map.insert('#', ());
            let data = Grid::<()>::from_str_by_map(input, &map).unwrap();
            assert_eq!(data.empty_rows(), vec![0]);
        }

        #[test]
        fn empty_rows_two() {
            let input = "...\n.x.\n...";
            let mut map: HashMap<char, ()> = HashMap::new();
            map.insert('x', ());
            let data = Grid::<()>::from_str_by_map(input, &map).unwrap();
            assert_eq!(data.empty_rows(), vec![-1, 1]);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }
    }
}
