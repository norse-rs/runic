//! Analytic coverage based on distance from circle center.

use crate::{math::*, Filter, RelativeBounds};

pub struct RadialBoxFilter {
    pub radius: f32,
}

impl Filter for RadialBoxFilter {
    fn name(&self) -> String {
        "RadialBox".into()
    }

    fn pdf(&self, x: f32) -> f32 {
        todo!()
    }

    fn cdf(&self, x: f32) -> f32 {
        let d = clamp(-x / self.radius, -1.0, 1.0);
        let triangle = (1.0 - d * d).sqrt() * d;
        let segment = d.acos();
        (segment - triangle) / std::f32::consts::PI
    }

    fn relative_bounds(&self, (x, y): (f32, f32)) -> RelativeBounds {
        let start_x = (x - self.radius).floor() as i32;
        let end_x = (x + self.radius).ceil() as i32;

        let start_y = (y - self.radius).floor() as i32;
        let end_y = (y + self.radius).ceil() as i32;

        RelativeBounds {
            x: start_x..=end_x,
            y: start_y..=end_y,
        }
    }
}
