use crate::{RelativeBounds, Filter};

pub struct LanzcosFilter {
    pub a: f32,
}

impl LanzcosFilter {
    fn sinc(x: f32) -> f32 {
        let x_pi = x * std::f32::consts::PI;
        x_pi.sin() / x_pi
    }
}

impl Filter for LanzcosFilter {
    fn name(&self) -> String { format!("Lanzcos {}", self.a) }

    fn pdf(&self, x: f32) -> f32 {
        if x == 0.0 {
            1.0
        } else if x > -self.a && x < self.a {
            Self::sinc(x) * Self::sinc(x / self.a)
        } else {
            0.0
        }
    }

    fn cdf(&self, _x: f32) -> f32 {
        todo!()
    }

    fn relative_bounds(&self, (x, y): (f32, f32)) -> RelativeBounds {
        let start_x = (x - self.a).floor() as i32;
        let end_x = (x + self.a).ceil() as i32;

        let start_y = (y - self.a).floor() as i32;
        let end_y = (y + self.a).ceil() as i32;

        RelativeBounds {
            x: start_x..=end_x,
            y: start_y..=end_y,
        }
    }
}
