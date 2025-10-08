use crate::Coordinate;
use crate::Positioned;
#[derive(Debug, Clone)]
pub enum GridCoordinate<T> {
    Empty(Coordinate),
    Object(T),
}

impl<T: Positioned + Clone> Positioned for GridCoordinate<T> {
    fn position(&self) -> &Coordinate {
        match self {
            GridCoordinate::Empty(coordinate) => coordinate.position(),
            GridCoordinate::Object(object) => object.position(),
        }
    }
}
