use crate::Positioned;
pub trait Mover: Positioned {
    fn set_coordinate<C: Positioned>(&mut self, coordinate: &C);
}
