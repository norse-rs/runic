use crate::{Filter, math::*};

#[derive(Debug, Clone, Copy)]
pub enum Colorspace {
    Linear,
    Srgb,
}

pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Self {
        Frame {
            width,
            height,
            data: vec![0; (width * height) as _],
        }
    }

    pub fn reconstruct(&mut self, framebuffer: &Framebuffer, filter: & dyn Filter, colorspace: Colorspace) {
        assert_eq!(self.width, framebuffer.width);
        assert_eq!(self.height, framebuffer.height);
        assert!(framebuffer.is_complete());

        let relative_bounds = filter.relative_bounds((0.5, 0.5));

        let layer_size = self.width * self.height;
        let num_samples = framebuffer.sample_pos.len();

        for y in 0..self.height {
            for x in 0..self.width {
                let mut acc_sample = 0.0;
                let mut acc_weight = 0.0;

                let bounds = relative_bounds.offset(x, y, self.width, self.height);
                for iy in bounds.y.clone() {
                    for ix in bounds.x.clone() {
                        let offset = num_samples * (iy * self.width + ix) as usize;
                        for (sample_id, sample_pos) in framebuffer.sample_pos.iter().enumerate() {
                            let id = sample_id + offset;
                            let sample = framebuffer.samples[id];

                            let dx = ix as i32 - x as i32;
                            let dy = iy as i32 - y as i32;
                            let weight = filter.pdf(sample_pos.x() - 0.5 + dx as f32) * filter.pdf(sample_pos.y() - 0.5 + dy as f32); // 2d separable filter

                            acc_sample += sample * weight;
                            acc_weight += weight;
                        }
                    }
                }

                let coverage = if acc_weight > 0.0 {
                    acc_sample / acc_weight
                } else {
                    0.0
                };

                let coverage = 0.5 * coverage + 0.5; // post process [-1.0, 1.0] -> [0.0, 1.0]
                let coverage = clamp(coverage, 0.0, 1.0);

                let opacity = match colorspace {
                    Colorspace::Linear => coverage,
                    Colorspace::Srgb => linear_to_srgb(coverage),
                };

                let value = (std::u8::MAX as f64 * opacity as f64) as u32;
                let i = /* (self.height - y - 1)*/ y * self.width + x; // y -flip
                self.data[i as usize] =
                    0xFF << 24 | value << 16 | value << 8 | value << 0;
            }
        }
    }
}

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub sample_pos: Vec<glam::Vec2>,
    pub samples: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Framebuffer {
            width,
            height,
            sample_pos: Vec::new(),
            samples: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.sample_pos.clear();
        self.samples.clear();
    }

    pub fn clear(&mut self) {
        self.samples.clear();
    }

    pub fn add_sample_pos(&mut self, position: glam::Vec2) {
        self.sample_pos.push(position);
        self.samples
            .extend(&vec![0.0; (self.width * self.height) as _]);
    }

    pub fn num_texels(&self) -> usize {
        (self.width * self.height) as _
    }

    pub fn is_complete(&self) -> bool {
        self.samples.len() == (self.num_texels() * self.sample_pos.len())
    }
}
