use crate::{
    math::*, rasterize_each_with_bias, Curve, Filter, Framebuffer, Rasterizer, Rect, Segment,
};

pub struct GouacheRasterizer<F: Filter> {
    pub filter: F,
}

impl<F: Filter> Rasterizer for GouacheRasterizer<F> {
    fn name(&self) -> String {
        format!("GouacheRasterizer :: {}", self.filter.name())
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

            // Antialiasing improvements based on https://github.com/glowcoil/gouache
            // Tangent based weighting and clamping of sampling points

            for curve in path {
                match curve {
                    Curve::Line { p0, p1 } => {
                        let p0 = *p0 - pos_curve;
                        let p1 = *p1 - pos_curve;

                        let xx0 = clamp(p0.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                        let xx1 = clamp(p1.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                        let xx = xx1 - xx0;

                        let mut cy = 0.0;

                        if p0.y().max(p1.y()) > -0.5 * dxdy.y() {
                            if xx != 0.0 && p0.y().min(p1.y()) < 0.5 * dxdy.y() {
                                let t = line_raycast(p0.x(), p1.x(), 0.5 * (xx0 + xx1)); // raycast y direction at sample pos
                                let d = line_eval(p0.y(), p1.y(), t) / dxdy.y(); // get x value at ray intersection
                                let tangent = p1 - p0;
                                let f = d * tangent.x().abs() / tangent.length();
                                coverage_y += xx as f32 * self.filter.cdf(f);
                            } else {
                                cy = 1.0;
                            }
                        }

                        cy *= xx;
                        coverage_y += cy;
                    }
                    Curve::Quad { p0, p1, p2 } => {
                        let p0 = *p0 - pos_curve;
                        let p1 = *p1 - pos_curve;
                        let p2 = *p2 - pos_curve;

                        let xx0 = clamp(p0.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                        let xx1 = clamp(p2.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                        let xx = xx1 - xx0;

                        let mut cy = 0.0;
                        if p0.y().max(p2.y()) > -0.5 * dxdy.y() {
                            if xx != 0.0 && p0.y().min(p2.y()) < 0.5 * dxdy.y() {
                                let t = quad_raycast(p0.x(), p1.x(), p2.x(), 0.5 * (xx0 + xx1));
                                let d = quad_eval(p0.y(), p1.y(), p2.y(), t) / dxdy.y();

                                let tangent = (p1 - p0) * (1.0 - t) + (p2 - p1) * t;
                                let f = d * tangent.x().abs() / tangent.length();
                                cy = self.filter.cdf(f);
                            } else {
                                cy = 1.0;
                            }
                        }
                        cy *= xx;
                        coverage_y += cy;
                    }
                }
            }

            coverage_y
        });
    }
}
