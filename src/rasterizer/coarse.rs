use crate::{
    line_eval, rasterize_each_with_bias, Curve, Framebuffer, Rasterizer, Rect, SampleId, Segment,
};

pub struct CoarseRasterizer;

pub type CoarsePath = Vec<Curve>;

impl Rasterizer for CoarseRasterizer {
    type Path = CoarsePath;

    fn create_path(&mut self, segments: &[Segment]) -> Self::Path {
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
        (sample_id, framebuffer): (SampleId, &mut Framebuffer),
        rect: Rect,
        path: &Self::Path,
    ) {
        rasterize_each_with_bias(
            (1.0, 1.0),
            (sample_id, framebuffer),
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
                                let t = (0.0 - p0.y()) / (p1.y() - p0.y());
                                let x = line_eval(p0.x(), p1.x(), t) / dxdy.x();
                                coverage += sign_y as f32 * (0.5 - x).min(1.0).max(0.0);
                            }
                        }
                        Curve::Quad { .. } => todo!(),
                    }
                }

                coverage.abs()
            },
        );
    }
}
