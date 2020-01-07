use crate::{Framebuffer};

pub struct UniformSampler {
    pub nx: usize,
    pub ny: usize,
}

impl UniformSampler {
    pub fn populate(&self, framebuffer: &mut Framebuffer) {
        let dx = 1.0 / self.nx as f32;
        let dy = 1.0 / self.ny as f32;

        for y in 0..self.ny {
            for x in 0..self.nx {
                framebuffer.add_sample_pos(glam::vec2((x as f32 + 0.5) * dx, (y as f32 + 0.5) * dy));
            }
        }
    }
}