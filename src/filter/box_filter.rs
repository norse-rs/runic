use crate::math::*;

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

    // Probability distribution function
    pub fn pdf(&self, _: f32) -> f32 {
        1.0 / (self.max - self.min)
    }

    // Cumulative distribution function
    pub fn cdf(&self, x: f32) -> f32 {
        clamp((x - self.min) / (self.max - self.min), 0.0, 1.0)
    }
}