mod coarse;
mod distance;

pub use coarse::*;
pub use distance::*;

use crate::{Curve, Extent, FillRect, Framebuffer, Offset, Rect, SampleId, Segment};

pub trait Rasterizer {
    fn name(&self) -> &'static str;

    fn create_path(&mut self, segments: &[Segment]) -> Vec<Curve>;

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

    fn cmd_draw(&mut self, frame: (SampleId, &mut Framebuffer), rect: Rect, path: &[Curve]);
}

pub(crate) fn rasterize_each_with_bias<F>(
    bias: (f32, f32),
    (sample_id, framebuffer): (SampleId, &mut Framebuffer),
    rect: Rect,
    coverage: F,
) where
    F: Fn(glam::Vec2, glam::Vec2) -> f32,
{
    let fill_rect = FillRect::new_with_bias(
        bias,
        rect.offset_local,
        rect.extent_local,
        framebuffer.width,
        framebuffer.height,
    );
    let width = framebuffer.width;
    let sample_pos = framebuffer.sample_pos[sample_id];
    let samples = framebuffer.get_samples_by_id(sample_id);

    let dxdy = rect.curve_dxdy();

    for y in fill_rect.y0..=fill_rect.y1 {
        for x in fill_rect.x0..=fill_rect.x1 {
            let pos_local = glam::Vec2::new(x as f32, y as f32) + sample_pos;
            let pos_curve = rect.local_to_curve(pos_local);

            let i = y * width + x;
            samples[i as usize] = coverage(pos_curve, dxdy);
        }
    }
}
