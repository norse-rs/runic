use crate::{RelativeBounds, Filter, math::*};

pub struct Smoothstep {
    pub e0: f32,
    pub e1: f32,
}

impl Smoothstep {
    fn t(&self, x: f32) -> f32 {
        clamp((x - self.e0) / (self.e1 - self.e0), 0.0, 1.0)
    }
}

impl Filter for Smoothstep {
    fn name(&self) -> String { "Smoothstep".into() }

    fn pdf(&self, x: f32) -> f32 {
        let t = self.t(x);
        if t >= 0.0 && t <= 1.0 {
            6.0 * (t - t*t)
        } else {
            0.0
        }
    }

    fn cdf(&self, x: f32) -> f32 {
        let t = self.t(x);
        if t < 0.0 { 0.0 } else if 1.0 <= t { 1.0 }else {
            3.0 * t * t - 2.0 * t * t * t
        }
    }

    fn relative_bounds(&self, (x, y): (f32, f32)) -> RelativeBounds {
        todo!()
    }
}
