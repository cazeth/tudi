## [0.4.1] - 2026-11-09

### Features

- *(axis-count)* Add From<AxisCount> for u64
- *(grid)* Derive clone
- *(axis)* Create axis as pub type
- *(Coordinate)* Add const Coordinate::MAX and const Coordinate::MIN
- [**breaking**] Replace u32 with AxisLength as function arguments in Positioned::coordinate_in_direction and BoundedMovingObject::move_in_current_direction
- *(out-of-bounds-error)* Add method with_single_direction
- *(out-of-bounds-error)* Add method add_second_direction

### Bug Fixes

- *(out-of-bounds-error)* Fix fn position

### Refactor

- Replace AxisCount::as_u64 with u64::from
- *(grid-coordinate)* Remove unecessary Clone trait bound
- *(axis-count)* Redo impl Debug
- *(axis-count)* Refactor x_min and x_max in bounded
- *(origin-centered-bounds)* Replace as_u64 with as_u32
- *(grid)* Remove cross.txt and edges_only.txt imports for tests
- Remove printlns in tests
- *(bounded)* Make filter more concise
- *(grid)* Simplify Tryfrom<Vec<Vec<T>>>
- *(grid)* Rename test fn symmetric_shape_should_transpose_to_itself
- *(moving-object)* Refactor unit tests
- *(bounded)* Replace i32::MIN/MAX with Coordinate::MIN/MAX
- *(grid)* Replace variable count with n inside test helper function
- *(bounded)* Add test method check_ok_coordinate_to_index
- *(out-of-bounds-error)* Refactor error message into a separate method
- *(bounded)* Simplify fn index_to_coordinate
- *(out-of-bounds-error)* Use i128 internally for x- and y-position
- *(bounded)* Add comment
- *(bounds)* Clean up use statements
- *(bounded)* Rework fn to_grid_like
- *(bounded)* Replace generic parameter T with C for consistency
- *(positioned)* Rename cord with coord

### Documentation

- *(bounded-moving-object)* Add docs for fn turn_toward
- *(bounded-moving-object)* Add docs for fn turn
- *(bounded-moving-object)* Add docs for fn move_in_current_direction
- *(AxisCount)* Add intradoc links
- *(moving_object)* Document movement panics
- *(Positioned)* Document movement panics
- *(out-of-bounds-error)* Add overview docs
- *(Positioned)* Improve docs for Positioned::coordinate_in_direction
- *(bounded)* Improve docs for Bounded::coordinate_to_index
- *(bounded)* Fix doc typo in Bounded::x/y_geometric_len
- *(bounded)* Improve docs of fn index_to_coordinate
- *(grid)* Add docs to test helper method empty_grid
- *(out-of-bounds-error)* Add docs to fn new
- *(out-of-bounds-error)* Create docs for fn position
- *(out-of-bounds-error)* Fix overview docs
- *(bounded)* Improve docs

### Testing

- *(grid)* Remove unit tests with imports
- *(grid)* Add double transpose test
- *(grid)* Fix assert_centered_around_origin
- *(coordinate)* Replace move_in_direction with coordinate_in_direction
- *(grid)* Add unit tests for empty_rows
- *(grid)* Add unit tests for fn coordinate_to_index
- *(out-of-bounds-error)* Add unit tests
- *(grid)* Add unit tests for fn index_to_coordinate
- *(bounds)* Remove unit test for invalid out-of-bounds error behavior.
- *(bounds)* Add to_matrix_like unit test
- *(coordinate)* Add unit test for fn manhattan_distance_to_origin

## [0.3.2] - 2026-08-30

### Features

- *(axis_count)* Add const as_u32
- *(AxisLength)* Create

### Bug Fixes

- *(bounded)* Bug
- *(bounded)* Fix to_grid_like

### Other

- *(refactor)* Remove expect clippy lints

### Refactor

- *(bounded-moving-object)* Change test helper fn changing_bounds::set
- Add global warn(missing_debug_implementation)
- *(bounded)* Fix clippy
- Allow must use candidate globally
- *(bounded-moving-object)* Simplify calls
- *(origin-centered-bounds)* Replace is_multiple_of
- *(grid)* Simplify test
- *(bounded)* Replace is_multiple_of
- *(origin-centered-bounded)* Replace use of is_multiple_of
- *(grid)* Remove let chain

### Documentation

- *(grid)* Add docs to move_elements_above/below_in_direction
- *(bounded-moving-object)* Fix docs
- *(grid)* Update docs for fn element

### Testing

- *(bounds)* Add unit tests for fn to_grid_like
- *(bounded-moving-object)* Add and refactor unit tests

### Miscellaneous Tasks

- *(clippy)* Allow Rust 1.85 compatibility lints
- Declare Rust 1.85 MSRV
- Add CI

## [0.3.1] - 2026-08-16

### Bug Fixes

- *(grid)* Return out of bounds error in get_mut_element instead of panicking
- *(grid)* Fix row_filter_move_elements_in_direction
- *(bounded-moving-object)* Fix typo in error message

### Refactor

- *(coordinate)* Refactor method move_in_direction
- *(grid)* Fix typo in unit test name
- *(element)* Simplify
- *(coordinate)* Refactor fn difference

### Documentation

- Fix typos
- *(grid)* Add docs for get_mut_element
- *(bounded)* Update docs out_of_bounds_direction

### Testing

- *(grid)* Add unit tests for get_mut_element
- *(grid)* Add unit tests for move_elements_above_row

## [0.3.0] - 2026-08-13

### 🚀 Features

- *(Grid)* Create fn with_count
- *(grid)* [**breaking**] Make Grid::from_bounds fallible
- *(grid)* Deprecate Grid::new
- *(grid)* Deprecate fn print_properties
- *(grid)* Add construction macro
- *(positioned)* [**breaking**] Change return type of direction_toward
- Misc derive trait implementations
- *(Bounded)* Add out-of-bounds directions
- *(direction)* Implement Display for direction enums
- *(Coordinate)* Implement Display and arithmetic traits
- *(GridError)* Include out-of-bounds directions
- *(Bounds)* Add infallible constructor Bounds::from_boundaries
- *(axis-count)* Add axis count type
- *(bounds)* Deprecate legacy constructor
- *(bounded-moving-object)* Derive Copy
- *(Direction)* Create Vertical and HorizontalDirection

### 🐛 Bug Fixes

- *(Grid)* [**breaking**] Handle empty str argument case in fn from_str_by_map
- *(grid)* [**breaking**] Handle empty case in TryFrom<Vec<Vec<Option>>>
- *(bounded-moving-object)* Normalize constructor boundaries
- *(bounded-moving-object)* Preserve boundaries when resizing

### 🚜 Refactor

- *(Grid)* Refactor tests
- *(OriginCenteredBounds)* Fix clippy
- *(Grid)* Refactor fn add_row
- *(Grid)* Remove unnecessary #[allow(unused)]
- *(Grid)* Tidy imports
- *(Grid)* Refactor constructor in test
- *(Grid)* Clippy code fix
- *(Positioned)* Fix comment typos
- *(Grid)* Rename tests to clearer names
- *(grid)* Fix compiler warnings
- *(BoundedMovingObject)* Drop Grid dependency from unit tests
- *(grid)* Refactor unit tests to use helper constructor functions
- *(Grid)* Simplify use declarations in module
- *(grid)* Drop redundant unit tests
- *(bounded)* Use is_multiple_of for boundary calculations
- *(grid)* Remove commented printlns! in fn empty_columns
- *(grid)* Simplify transpose iteration
- *(grid)* Replace T: Coordinate with T: () in unit tests
- *(grid)* Use checked storage helper in unit test
- *(test)* Satisfy clippy collapsible_if
- *(grid)* Document infallible transpose operations
- *(grid)* Refactor iter_mut_elements_new, iter_mut_new and iter_elements_new
- *(BoundedMovingObject)* Remove unnecessary #[allow(deprecated)] in BoundedMovingObject
- *(Coordinate)* Implement Add in terms of AddAssign
- *(BoundedMovingObject)* Rustfmt
- *(grid)* Switch from using Bounds::new to Bounds::from_boundaries
- Rustfmt
- *(bounded-moving-object)* Simplify initialization
- *(bounded-moving-object)* Correct inaccurate southeast corner wording
- *(origin-centered-bounds)* Construct from boundaries
- *(bounded)* Remove commented code
- *(grid)* Simplify fn from_bounds
- *(grid)* Allocate vec once upfront in fn with_count
- *(origin-centered-bounds)* Refactor Self::new
- *(origin-centered-bounds)* Improve impl TryFrom<Bounds>
- Use check_x/y_count in tests

### 📚 Documentation

- *(Bounds)* Improve docs
- *(Grid)* Improve docs
- *(Grid)* Improve docs
- *(Grid)* Add docs
- *(Bounded)* Improve docs
- *(Positioned)* Improve docs
- *(Bounded)* Improve docs
- *(Direction)* Typos
- *(Grid)* Add overview docs
- *(bounded)* Remove grid from doc tests
- *(grid)* Misc improvements
- *(grid)* Replace fn new with with_count in examples
- *(grid)* Link GridError documentation to Grid
- *(grid)* Document test helper panics
- *(bounded)* Clarify border-check documentation
- *(OriginCenteredBounds)* Language improvement
- *(bounded-moving-object)* Remove redundant constructor example
- *(grid)* Update construction to use macro
- *(grid)* Simplify doc examples
- *(grid)* Remove outdated documentation
- *(grid)* Update documentation for with_count
- *(grid)* Add capacity to overview
- *(grid)* Update documentation for fn remove_element
- *(bounded)* Update docs for fn index_to_coordinate
- *(grid)* Add ticks to TryFrom<Vec<Vec<Option<T>>> docs

### 🧪 Testing

- *(Grid)* Add test fn char_not_in_map
- *(grid)* Add x/y count unit tests
- *(grid)* Add unit tests for is_within_bounds
- *(grid)* Fix test bug
- *(grid)* Improve error unit test
- *(grid)* Verify coordinate coverage invariants
- *(grid)* Cover invalid store operations
- *(grid)* Cover Grid::from_bounds
- *(OriginCenteredBounds)* Add unit test for smallest OriginCenteredBounds
- *(OriginCenteredBounds)* Add tests on create from bounds
- *(GridError)* Verify out-of-bounds directions
- *(bounded)* Share boundary assertions
- *(origin-centered-bounds)* Use boundary constructor
- Centralize count assertions
- *(origin-centered-bounds)* Improve unit test
- *(origin-centered-bounds)* Add unit test
- *(grid)* Add geometric len unit tests to Grid
- *(bounds)* Add unit tests for large and small values
- *(origin-centered-bounds)* Add unit tests

## [0.2.0] - 2025-12-18

### 🚀 Features

- [**breaking**] Change usize to u32 in signatures for multiple methods
- *(Bounded)* Add provided method bounded_neighbors_to

### 🚜 Refactor

- *(BoundedMovingObject)* Rename internal fn get_bounded_neighbors

### 📚 Documentation

- *(OriginCenteredBounds)* Improve docs
- *(Grid)* Misc improvements
- *(Bounded)* Fix incorrect documentation for fn bounded_neighbors

### 🧪 Testing

- *(Grid)* Refactor check_x_count and check_y_count
- *(Grid)* Add tests for fn check_bounded_to

### ⚙️ Miscellaneous Tasks

- Update version
## [0.1.0] - 2025-10-25

### 🚀 Features

- *(OutofBoundsError)* Derive Clone
- *(GridError)* Derive Clone

### 🐛 Bug Fixes

- *(Grid)* Change the error variant returned in case of out of bounds move

### 🚜 Refactor

- *(Positioned)* Allow clippy::enum_glob_use

### 📚 Documentation

- *(OriginCenteredBounds)* Update docs
- *(Positioned)* Fix docs

### 🧪 Testing

- *(Grid)* Refactor tests
- *(Grid)* Improve test grid constructor function
- *(Grid)* Add tests for method move_elements_in_direction

### ⚙️ Miscellaneous Tasks

- Update Cargo.toml
