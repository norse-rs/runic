use crate::sample::SampleId;

pub enum ReconstructionFilter {
    Box,
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

    pub fn reconstruct(&mut self, _filter: ReconstructionFilter, framebuffer: &Framebuffer) {
        assert_eq!(self.width, framebuffer.width);
        assert_eq!(self.height, framebuffer.height);
        assert!(framebuffer.is_complete());

        let layer_size = self.width * self.height;
        for (sample_id, _sample_pos) in framebuffer.sample_pos.iter().enumerate() {
            for i in 0..layer_size as usize {
                let id = sample_id * layer_size as usize + i;
                let sample = framebuffer.samples[id];

                // TODO
                let clamped_sample = sample.min(1.0).max(0.0);
                let contribution = (std::u8::MAX as f64 * clamped_sample as f64) as u32;
                self.data[i] =
                    0xFF << 24 | contribution << 16 | contribution << 8 | contribution << 0;
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

    pub fn add_samples_pos(&mut self, position: glam::Vec2) {
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

    pub fn get_samples_by_id(&mut self, sample_id: SampleId) -> &mut [f32] {
        let num_texels = self.num_texels();
        let start = sample_id * num_texels;
        let end = (sample_id + 1) * num_texels;

        &mut self.samples[start..end]
    }
}
