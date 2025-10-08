use crate::bounded::Bounded;

pub trait DynamicallyBounded: Bounded {
    fn set_x_min_boundary(&mut self, boundary: i32) -> Result<i32, String>;
    fn set_x_max_boundary(&mut self, boundary: i32) -> Result<i32, String>;
    fn set_y_min_boundary(&mut self, boundary: i32) -> Result<i32, String>;
    fn set_y_max_boundary(&mut self, boundary: i32) -> Result<i32, String>;

    fn inherit_boundaries<B: Bounded>(&mut self, bounded: &B) -> Result<(), String> {
        self.set_x_min_boundary(bounded.x_min_boundary())?;
        self.set_y_min_boundary(bounded.y_min_boundary())?;
        self.set_y_max_boundary(bounded.y_max_boundary())?;
        self.set_x_max_boundary(bounded.x_max_boundary())?;
        Ok(())
    }
}
