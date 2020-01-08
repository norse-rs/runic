mod coarse;
mod distance;
mod analytic_box;

pub use coarse::*;
pub use distance::*;
pub use analytic_box::*;

use crate::{Curve, Extent, FillRect, Framebuffer, Offset, Rect, SampleId, Segment};

pub trait Rasterizer {
    fn name(&self) -> String;

    fn create_path(&mut self, segments: &[Segment]) -> Vec<Curve>;

    fn cmd_fill(
        &mut self,
        framebuffer: &mut Framebuffer,
        offset: Offset,
        extent: Extent,
        value: f32,
    ) {
        let fill_rect = FillRect::new(offset, extent, framebuffer.width, framebuffer.height);
        let width = framebuffer.width;
        let num_samples = framebuffer.sample_pos.len();

        for y in fill_rect.y0..=fill_rect.y1 {
            for x in fill_rect.x0..=fill_rect.x1 {
                for sample_id in 0..num_samples {
                    let i = sample_id + num_samples * (y * width + x) as usize;
                    framebuffer.samples[i as usize] = value;
                }
            }
        }
    }

    fn cmd_draw(&mut self, frame: &mut Framebuffer, rect: Rect, path: &[Curve]);
}

pub(crate) fn rasterize_each_with_bias<F>(
    bias: (f32, f32),
    framebuffer: &mut Framebuffer,
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
    let dxdy = rect.curve_dxdy();
    let num_samples = framebuffer.sample_pos.len();

    for y in fill_rect.y0..=fill_rect.y1 {
        for x in fill_rect.x0..=fill_rect.x1 {
            for (sample_id, sample_pos) in framebuffer.sample_pos.iter().enumerate() {
                let pos_local = glam::Vec2::new(x as f32, y as f32) + *sample_pos;
                let pos_curve = rect.local_to_curve(pos_local);

                let i = sample_id + num_samples * (y * width + x) as usize;
                framebuffer.samples[i] = coverage(pos_curve, dxdy);
            }
        }
    }
}
