use crate::{
    math::*, rasterize_each_with_bias, Curve, Filter, Framebuffer, Rasterizer, Rect, Segment,
};

pub struct HatiRasterizer<F: Filter> {
    pub filter: F,
}

impl<F: Filter> Rasterizer for HatiRasterizer<F> {
    fn name(&self) -> String {
        format!("HatiRasterizer :: {}", self.filter.name())
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
            let mut coverage = 0.0;
            let mut distance = 100000.0f32;

            for curve in path {
                match curve {
                    Curve::Line { p0, p1 } => {
                        let p0 = *p0 - pos_curve;
                        let p1 = *p1 - pos_curve;

                        let sign_y = (p1.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                        let dir = p1 - p0;
                        let dp = -p0;
                        let t = dir.dot(dp) / dir.dot(dir);
                        let n = (dp - dir * t) / dxdy;
                        let sign = n.x().signum();

                        let nd = (dp - dir * clamp(t, 0.0, 1.0)) / dxdy;
                        let d = nd.length();

                        coverage += sign_y as f32 * sign.min(0.0);
                        distance = distance.min(d);
                    }
                    Curve::Quad { p0, p1, p2 } => {
                        let p0 = *p0 - pos_curve;
                        let p1 = *p1 - pos_curve;
                        let p2 = *p2 - pos_curve;

                        let sign_y = (p2.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                        let t = quad_raycast(p0.y(), p1.y(), p2.y(), 0.0);
                        let dx = quad_eval(p0.x(), p1.x(), p2.x(), t) / dxdy.x();

                        let yy0 = clamp(-0.5 * dxdy.y(), p0.y(), p2.y());
                        let yy1 = clamp(0.5 * dxdy.y(), p0.y(), p2.y());

                        let ty0 = quad_raycast(p0.y(), p1.y(), p2.y(), yy0); // raycast y direction
                        let ty1 = quad_raycast(p0.y(), p1.y(), p2.y(), yy1); // raycast y direction

                        // xx = (p2.x() > 0.0) as i32 - (p0.x() > 0.0) as i32;
                        // yy = (p2.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                        // if yy != 0 {
                        //     let t = quad_raycast(p0.y(), p1.y(), p2.y(), 0.0);
                        //     let d = quad_eval(p0.x(), p1.x(), p2.x(), t) / dxdy.x();

                        //     coverage -= yy as f32 * d.signum().min(0.0);
                        //     a = d;
                        // }

                        // let d = match (xx == 0, yy == 0) {
                        //     (true, true) => quack,
                        //     (true, false) => a.abs(),
                        //     (false, true) => b.abs(),
                        //     (false, false) => (a * b).abs() / (2.0 * (a*a + b * b).sqrt()),
                        // };
                        // quack = quack.min(d);
                    }
                }
            }

            self.filter.cdf((2.0 * coverage - 1.0) * distance)
        });
    }
}
