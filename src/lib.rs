mod frame;
mod paths;
mod rasterizer;
mod sample;

pub use crate::frame::*;
pub use crate::paths::*;
pub use crate::rasterizer::*;
pub use crate::sample::*;

pub type Offset = glam::Vec2;
pub type Extent = glam::Vec2;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub offset_local: Offset,
    pub extent_local: Extent,
    pub offset_curve: Offset,
    pub extent_curve: Extent,
}

impl Rect {
    pub fn local_to_curve(&self, local: glam::Vec2) -> glam::Vec2 {
        let tx = if self.extent_local.x().abs() > 0.0 {
            (local.x() - self.offset_local.x()) / self.extent_local.x()
        } else {
            0.0
        };

        let ty = if self.extent_local.y().abs() > 0.0 {
            (local.y() - self.offset_local.y()) / self.extent_local.y()
        } else {
            0.0
        };

        glam::vec2(self.offset_curve.x() + tx * self.extent_curve.x(), self.offset_curve.y() + ty * self.extent_curve.y())
    }
}

pub struct FillRect {
    pub x0: u32,
    pub x1: u32,
    pub y0: u32,
    pub y1: u32,
}

impl FillRect {
    pub fn new(offset: Offset, extent: Extent, bound_x: u32, bound_y: u32) -> Self {
        let p0 = offset;
        let p1 = extent + offset;

        let x0 = (p0.x().floor().max(0.0) as u32).min(bound_x);
        let y0 = (p0.y().floor().max(0.0) as u32).min(bound_y);
        let x1 = (p1.x().ceil().max(0.0) as u32).min(bound_x);
        let y1 = (p1.y().ceil().max(0.0) as u32).min(bound_y);

        FillRect { x0, x1, y0, y1 }
    }
}
