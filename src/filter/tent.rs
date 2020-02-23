use crate::{Filter, RelativeBounds};

pub struct TentFilter;

impl Filter for TentFilter {
    fn name(&self) -> String {
        "Tent".into()
    }

    fn pdf(&self, x: f32) -> f32 {
        let value = if x < 0.0 { 1.0 + x } else { 1.0 - x };

        value.max(0.0)
    }

    fn cdf(&self, _x: f32) -> f32 {
        todo!()
    }

    fn relative_bounds(&self, (x, y): (f32, f32)) -> RelativeBounds {
        let start_x = (x - 1.0).floor() as i32;
        let end_x = (x + 1.0).ceil() as i32;

        let start_y = (y - 1.0).floor() as i32;
        let end_y = (y + 1.0).ceil() as i32;

        RelativeBounds {
            x: start_x..=end_x,
            y: start_y..=end_y,
        }
    }
}
