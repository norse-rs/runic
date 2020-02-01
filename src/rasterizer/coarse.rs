use crate::{
    math::*, rasterize_each_with_bias, Curve, Framebuffer, Rasterizer, Rect,
    Segment, Filter,
};

pub enum CoarseDirection {
    Y,
    X,
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

    fn cmd_draw(
        &mut self,
        framebuffer: &mut Framebuffer,
        rect: Rect,
        path: &[Curve],
    ) {
        rasterize_each_with_bias(
            (1.0, 1.0),
            framebuffer,
            rect,
            |pos_curve, dxdy| {
                let mut coverage_x = 0.0;
                let mut coverage_y = 0.0;

                // Antialiasing improvements based on https://github.com/glowcoil/gouache
                // Tangent based weighting and clamping of sampling points

                for curve in path {
                    match curve {
                        Curve::Line { p0, p1 } => {
                            let p0 = *p0 - pos_curve;
                            let p1 = *p1 - pos_curve;

                            let yy0 = clamp(p0.y(), -0.5 * dxdy.y(), 0.5 * dxdy.y());
                            let yy1 = clamp(p1.y(), -0.5 * dxdy.y(), 0.5 * dxdy.y());
                            let yy = yy1 - yy0;

                            if yy != 0.0 {
                                let t = line_raycast(p0.y(), p1.y(), 0.5 * (yy0 + yy1)); // raycast x direction at sample pos
                                let d = line_eval(p0.x(), p1.x(), t) / dxdy.x(); // get y value at ray intersection
                                let tangent = p1 - p0;
                                let f = d * tangent.y().abs() / tangent.length();
                                coverage_x += yy * self.filter.cdf(f);
                            }

                            let xx0 = clamp(p0.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                            let xx1 = clamp(p1.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                            let xx = xx1 - xx0;

                            if xx != 0.0 {
                                let t = line_raycast(p0.x(), p1.x(), 0.0); // raycast y direction at sample pos
                                let d = line_eval(p0.y(), p1.y(), t) / dxdy.y(); // get x value at ray intersection
                                let tangent = p1 - p0;
                                let f = d * tangent.x().abs() / tangent.length();
                                coverage_y += xx as f32 * self.filter.cdf(f);
                            }
                        }
                        Curve::Quad { p0, p1, p2 } => {
                            let p0 = *p0 - pos_curve;
                            let p1 = *p1 - pos_curve;
                            let p2 = *p2 - pos_curve;

                            let yy0 = clamp(p0.y(), -0.5 * dxdy.y(), 0.5 * dxdy.y());
                            let yy1 = clamp(p2.y(), -0.5 * dxdy.y(), 0.5 * dxdy.y());

                            let yy = yy1 - yy0;

                            if yy != 0.0 {
                                let t = quad_raycast(p0.y(), p1.y(), p2.y(), 0.5 * (yy0 + yy1));
                                let d = quad_eval(p0.x(), p1.x(), p2.x(), t) / dxdy.x();

                                let tangent = (p1 - p0) * (1.0 - t) + (p2 - p1) * t;
                                let f = d * tangent.y().abs() / tangent.length();
                                coverage_x += yy as f32 * self.filter.cdf(f);
                            }

                            let xx0 = clamp(p0.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                            let xx1 = clamp(p2.x(), -0.5 * dxdy.x(), 0.5 * dxdy.x());
                            let xx = xx1 - xx0;

                            if xx != 0.0 {
                                let t = quad_raycast(p0.x(), p1.x(), p2.x(), 0.0);
                                let d = quad_eval(p0.y(), p1.y(), p2.y(), t) / dxdy.y();

                                let tangent = (p1 - p0) * (1.0 - t) + (p2 - p1) * t;
                                let f = d * tangent.x().abs() / tangent.length();
                                coverage_y += xx * self.filter.cdf(f);
                            }
                        }
                    }
                }

                match self.direction {
                    CoarseDirection::X => coverage_y,
                    CoarseDirection::Y => -coverage_x,
                }
            },
        );
    }
}
