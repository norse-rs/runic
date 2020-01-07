use crate::{RelativeBounds, Filter, math::*};

pub struct BoxFilter {
    min: f32,
    max: f32,
}

impl BoxFilter {
    pub fn new(min: f32, max: f32) -> Self {
        BoxFilter {
            min, max
        }
    }
}

impl Filter for BoxFilter {
    fn name(&self) -> String { "Box".into() }

    fn pdf(&self, x: f32) -> f32 {
        if x >= self.min && x <= self.max {
            1.0 / (self.max - self.min)
        } else {
            0.0
        }
    }

    fn cdf(&self, x: f32) -> f32 {
        clamp((x - self.min) / (self.max - self.min), 0.0, 1.0)
    }

    fn relative_bounds(&self, (x, y): (f32, f32)) -> RelativeBounds {
        let start_x = (x + self.min).floor() as i32;
        let end_x = (x + self.max).ceil() as i32;

        let start_y = (y + self.min).floor() as i32;
        let end_y = (y + self.max).ceil() as i32;

        RelativeBounds {
            x: start_x..=end_x,
            y: start_y..=end_y,
        }
    }
}