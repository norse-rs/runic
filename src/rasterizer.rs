mod coarse;

pub use coarse::*;

use crate::{Extent, Framebuffer, Offset, Rect, SampleId, Segment};

pub trait Rasterizer {
    type Path;

    fn create_path(&mut self, segments: &[Segment]) -> Self::Path;

    fn cmd_fill(
        &mut self,
        frame: (SampleId, &mut Framebuffer),
        offset: Offset,
        extent: Extent,
        value: f32,
    );
    fn cmd_draw(&mut self, frame: (SampleId, &mut Framebuffer), rect: Rect, path: &Self::Path);
}
