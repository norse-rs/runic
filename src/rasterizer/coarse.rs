use crate::{
    math::*, rasterize_each_with_bias, Curve, Filter, Framebuffer, Rasterizer, Rect, Segment,
};

pub enum CoarseDirection {
    Y,
    X,
    XY,
}

pub struct CoarseRasterizer<F: Filter> {
    pub filter: F,
    pub direction: CoarseDirection,
}

impl<F: Filter> Rasterizer for CoarseRasterizer<F> {
    fn name(&self) -> String {
        format!("CoarseRasterizer :: {}", self.filter.name())
    }

    fn create_path(&mut self, segments: &[Segment]) -> Vec<Curve> {
        let mut curves = Vec::new();
        for segment in segments {
            for curve in segment {
                curves.push(*curve);
            }
        }
        curves
    }

    fn cmd_draw(&mut self, framebuffer: &mut Framebuffer, rect: Rect, path: &[Curve]) {
        rasterize_each_with_bias((1.0, 1.0), framebuffer, rect, |pos_curve, dxdy| {
            let mut coverage_x = 0.0;
            let mut coverage_y = 0.0;

            let mut quack = 100000.0f32;

            for curve in path {
                let mut a = 0.0;
                let mut b = 0.0;

                let mut xx = 0;
                let mut yy = 0;

                match curve {
                    Curve::Line { p0, p1 } => {
                        let p0 = *p0 - pos_curve;
                        let p1 = *p1 - pos_curve;

                        xx = (p1.x() > 0.0) as i32 - (p0.x() > 0.0) as i32;
                        yy = (p1.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                        if yy != 0 {
                            let t = line_raycast(p0.y(), p1.y(), 0.0); // raycast x direction at sample pos
                            let d = line_eval(p0.x(), p1.x(), t) / dxdy.x(); // get y value at ray intersection

                            coverage_x -= yy as f32 * d.signum().min(0.0);
                            a = d;
                        }

                        if xx != 0 {
                            let t = line_raycast(p0.x(), p1.x(), 0.0); // raycast y direction at sample pos
                            let d = line_eval(p0.y(), p1.y(), t) / dxdy.y(); // get x value at ray intersection

                            coverage_y += xx as f32 * d.signum().min(0.0);
                            b = d;
                        }
                    }
                    Curve::Quad { p0, p1, p2 } => {
                        let p0 = *p0 - pos_curve;
                        let p1 = *p1 - pos_curve;
                        let p2 = *p2 - pos_curve;

                        xx = (p2.x() > 0.0) as i32 - (p0.x() > 0.0) as i32;
                        yy = (p2.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                        if yy != 0 {
                            let t = quad_raycast(p0.y(), p1.y(), p2.y(), 0.0);
                            let d = quad_eval(p0.x(), p1.x(), p2.x(), t) / dxdy.x();

                            coverage_x -= yy as f32 * d.signum().min(0.0);
                            a = d;
                        }

                        if xx != 0 {
                            let t = quad_raycast(p0.x(), p1.x(), p2.x(), 0.0);
                            let d = quad_eval(p0.y(), p1.y(), p2.y(), t) / dxdy.y();

                            coverage_y += xx as f32 * d.signum().min(0.0);
                            b = d;
                        }
                    }
                }

                let d = match (xx == 0, yy == 0) {
                    (true, true) => quack,
                    (true, false) => a.abs(),
                    (false, true) => b.abs(),
                    (false, false) => (a * b).abs() / (2.0 * (a * a + b * b).sqrt()),
                };
                quack = quack.min(d);
            }

            match self.direction {
                CoarseDirection::X => self.filter.cdf((2.0 * coverage_x - 1.0)),
                CoarseDirection::Y => self.filter.cdf((2.0 * coverage_y - 1.0)),
                CoarseDirection::XY => self.filter.cdf((coverage_y + coverage_x - 1.0) * quack),
            }
        });
    }
}
