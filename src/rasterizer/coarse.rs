use crate::{
    math::*, rasterize_each_with_bias, Curve, Framebuffer, Rasterizer, Rect, SampleId,
    Segment, Filter,
};

pub struct CoarseRasterizer<F: Filter> {
    pub filter: F,
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
                let mut coverage = 0.0;

                for curve in path {
                    match curve {
                        Curve::Line { p0, p1 } => {
                            let p0 = *p0 - pos_curve;
                            let p1 = *p1 - pos_curve;

                            let sign_y = (p1.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                            if sign_y != 0 {
                                let tx = line_raycast(p0.y(), p1.y(), 0.0); // raycast x direction at sample pos
                                let dx = line_eval(p0.x(), p1.x(), tx) / dxdy.x(); // get y value at ray intersection
                                let t = self.filter.cdf(dx);
                                coverage -= sign_y as f32 * t;
                            }
                        }
                        Curve::Quad { p0, p1, p2 } => {
                            let p0 = *p0 - pos_curve;
                            let p1 = *p1 - pos_curve;
                            let p2 = *p2 - pos_curve;

                            // // intersection check
                            let sign_x = (p2.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                            let a = p0 - 2.0 * p1 + p2;
                            let b = p0 - p1;
                            let c = p0;

                            // quad raycast
                            let dscr_sq = (b.y() * b.y() - a.y() * c.y());
                            let tx = (b.y() + sign_x as f32 * dscr_sq.sqrt()) / a.y();

                            let dx = ((a.x() * tx - 2.0 * b.x()) * tx + c.x()) / dxdy.x(); // quad eval

                            coverage -= sign_x as f32 * self.filter.cdf(dx);
                        }
                    }
                }

                coverage
            },
        );
    }
}
