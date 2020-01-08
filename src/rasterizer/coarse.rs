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
                        Curve::Quad { .. } => todo!(),
                    }
                }

                coverage
            },
        );
    }
}
