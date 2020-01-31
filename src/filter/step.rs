use crate::{RelativeBounds, Filter};

/// Heaviside
pub struct StepFilter;

impl Filter for StepFilter {
    fn name(&self) -> String { "Step".into() }

    fn pdf(&self, x: f32) -> f32 {
        // delta distribution
        if x == 0.0 { 1.0 } else { 0.0 }
    }

    fn cdf(&self, x: f32) -> f32 {
        if x < 0.0 { 0.0 } else { 1.0 }
    }

    fn relative_bounds(&self, (_x, _y): (f32, f32)) -> RelativeBounds {
        todo!()
    }
}
