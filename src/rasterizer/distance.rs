use crate::{Curve, Framebuffer, Rasterizer, Rect, SampleId, Segment, rasterize_each_with_bias};

pub struct DistanceRasterizer;

pub type DistancePath = Vec<Curve>;

impl Rasterizer for DistanceRasterizer {
    type Path = DistancePath;

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
                let mut distance = 1000.0f32;

                let mut coverage = 0.0;

                for curve in path {
                    match curve {
                        Curve::Line { p0, p1 } => {
                            // TODO

                            let p0 = *p0;
                            let p1 = *p1;

                            let sign_y = (p1.y() > p0.y()) as i32 - (p0.y() > p1.y()) as i32;

                            let dir = p1 - p0;
                            let dp = pos_curve - p0;
                            let t = (dir.dot(dp) / dir.dot(dir)).min(1.0).max(0.0);
                            let n = (dp - dir * t) / dxdy;
                            let d = n.length() * n.x().signum();

                            coverage += sign_y as f32 * (d).min(1.0).max(0.0);

                            distance = distance.min(d);
                        }
                        Curve::Quad { .. } => todo!(),
                    }
                }

                // let coverage = (1.0 - distance).max(0.0) / dxdy.length();

                coverage
            },
        );
    }
}
