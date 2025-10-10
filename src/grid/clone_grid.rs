use super::{Grid, GridCreationError};
use std::collections::HashMap;

impl<T: Clone> Grid<T> {
    /// Creates a grid from a str where each lines represents a row. Each character in the string
    /// is mapped to a grid element or an empty coordinate according to the provided hashmap.
    ///
    /// # Panics
    /// This method panics if the row in the input str are of different lengths.
    /// This method panics if there is a char in the string that is not represented in the map.
    ///
    pub fn from_str_to_unwrapped_with_borrowed_map(
        input: &str,
        map: &HashMap<char, Option<T>>,
    ) -> Result<Grid<T>, GridCreationError> {
        let mut char_data = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let data = char_data
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .map(|c| map.get(c).unwrap().clone())
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

    pub mod constructor_tests {
        use super::*;

        #[test]
        fn new_from_str_test() {
            let input = "...";
            let mut map: HashMap<char, Option<usize>> = HashMap::new();
            map.insert('.', None);
            let data = Grid::<usize>::from_str_to_unwrapped_with_borrowed_map(input, &map).unwrap();
            assert_eq!(data.x_count(), 3);
            assert_eq!(data.y_count(), 1);
            assert_eq!(data.iter_elements_new().count(), 0);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }

        #[test]
        fn new_from_str_test_two() {
            let input = ".x.";
            let mut map: HashMap<char, Option<usize>> = HashMap::new();
            map.insert('.', None);
            map.insert('x', Some(1));
            let data = Grid::<usize>::from_str_to_unwrapped_with_borrowed_map(input, &map).unwrap();
            assert_eq!(data.x_count(), 3);
            assert_eq!(data.y_count(), 1);
            assert_eq!(data.iter_elements_new().count(), 1);
            assert_eq!(*data.element(&Coordinate::default()).unwrap(), 1);
        }

        #[test]
        fn test_new_from_str_unwrapped_with_empty() {
            let input = "...";
            let mut map: HashMap<char, Option<usize>> = HashMap::new();
            map.insert('.', None);
            let data = Grid::<usize>::from_str_to_unwrapped_with_borrowed_map(input, &map).unwrap();
            assert_eq!(data.x_count(), 3);
            assert_eq!(data.y_count(), 1);
            assert_eq!(data.iter_elements_new().count(), 0);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }

        #[test]
        fn new_from_str_unwrapped_should_panic_when_rows_are_different_sizes() {
            let input = "...\n....";
            let mut map: HashMap<char, Option<usize>> = HashMap::new();
            map.insert('.', None);
            let res = Grid::<usize>::from_str_to_unwrapped_with_borrowed_map(input, &map);
            assert!(res.is_err())
        }

        #[test]
        fn empty_rows_one() {
            let input = ".#.\n...";
            let mut map: HashMap<char, Option<()>> = HashMap::new();
            map.insert('.', None);
            map.insert('#', Some(()));
            let data = Grid::<()>::from_str_to_unwrapped_with_borrowed_map(input, &map).unwrap();
            assert_eq!(data.empty_rows(), vec![0]);
        }

        #[test]
        fn empty_rows_two() {
            let input = "...\n.x.\n...";
            let mut map: HashMap<char, Option<()>> = HashMap::new();
            map.insert('.', None);
            map.insert('x', Some(()));
            let data = Grid::<()>::from_str_to_unwrapped_with_borrowed_map(input, &map).unwrap();
            assert_eq!(data.empty_rows(), vec![-1, 1]);
            assert_coordinate_coverage(&data);
            assert_centered_around_origin(&data);
        }
    }
}
