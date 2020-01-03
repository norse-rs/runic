use crate::frame::Framebuffer;
use crate::paths::Segment;
use crate::sample::SampleId;
use crate::Rasterizer;
use crate::{Extent, Offset, Rect, FillRect, Curve};

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

    fn cmd_fill(
        &mut self,
        (sample_id, framebuffer): (SampleId, &mut Framebuffer),
        offset: Offset,
        extent: Extent,
        value: f32,
    ) {
        let fill_rect = FillRect::new(offset, extent, framebuffer.width, framebuffer.height);
        let width = framebuffer.width;
        let samples = framebuffer.get_samples_by_id(sample_id);
        for y in fill_rect.y0..=fill_rect.y1 {
            for x in fill_rect.x0..=fill_rect.x1 {
                let i = y * width + x;
                samples[i as usize] = value;
            }
        }
    }

    fn cmd_draw(
        &mut self,
        (sample_id, framebuffer): (SampleId, &mut Framebuffer),
        rect: Rect,
        path: &Self::Path,
    ) {
        let fill_rect = FillRect::new(
            rect.offset_local,
            rect.extent_local,
            framebuffer.width,
            framebuffer.height,
        );
        let width = framebuffer.width;
        let sample_pos = framebuffer.sample_pos[sample_id];
        let samples = framebuffer.get_samples_by_id(sample_id);

        for y in fill_rect.y0..=fill_rect.y1 {
            for x in fill_rect.x0..=fill_rect.x1 {
                let pos_local = glam::Vec2::new(x as f32, y as f32) + sample_pos;
                let pos_curve = rect.local_to_curve(pos_local);

                let i = y * width + x;
                samples[i as usize] = pos_curve.x() / 100.0;
            }
        }
    }
}
