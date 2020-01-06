use crate::{rasterize_each_with_bias, Curve, Framebuffer, Rasterizer, Rect, SampleId, Segment, math::*};

pub struct DistanceRasterizer;

impl Rasterizer for DistanceRasterizer {
    fn name(&self) -> &'static str { "DistanceRasterizer" }

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
        (sample_id, framebuffer): (SampleId, &mut Framebuffer),
        rect: Rect,
        path: &[Curve],
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

                            let dir = p1 - p0;
                            let dp = -p0;
                            let t = (dir.dot(dp) / dir.dot(dir)).min(1.0).max(0.0);
                            let n = (dp - dir * t) / dxdy;
                            let d = n.length() * n.x().signum();

                            coverage -= sign_y as f32 * box_1d(-d, -0.7, 0.7);
                        }
                        Curve::Quad { .. } => todo!(),
                    }
                }

                coverage
            },
        );
    }
}
