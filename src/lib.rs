mod frame;
mod paths;
mod rasterizer;
mod sample;
mod rect;
mod polynomial;
mod math;

pub use crate::frame::*;
pub use crate::paths::*;
pub use crate::rasterizer::*;
pub use crate::sample::*;
pub use crate::rect::*;
pub use crate::polynomial::*;
pub use minifb::*;

pub type Scene = fn(&mut dyn Rasterizer, &mut Framebuffer);

pub struct App {
    width: u32,
    height: u32,
    frame: Frame,
    framebuffer: Framebuffer,
    window: Window,
    rasterizers: Vec<(Key, Box<dyn Rasterizer>)>,
    active_rasterizer: Option<usize>,
    scenes: Vec<(Key, Scene)>,
    active_scene: Option<usize>,
}

impl App {
    pub fn new(width: u32, height: u32, scale: Scale) -> Self {
        let frame = Frame::new(width, height);
        let framebuffer = Framebuffer::new(width, height);

        let mut window = Window::new(
            "furu",
            width as _,
            height as _,
            WindowOptions {
                borderless: false,
                title: true,
                resize: false,
                scale,
                scale_mode: ScaleMode::AspectRatioStretch,
            },
        )
        .unwrap();

        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        App {
            width,
            height,
            frame,
            framebuffer,
            window,
            rasterizers: Vec::new(),
            active_rasterizer: None,
            scenes: Vec::new(),
            active_scene: None,
        }
    }

    pub fn add_rasterizer<R: Rasterizer + 'static>(&mut self, key: Key, rasterizer: R) {
        if self.active_rasterizer.is_none() {
            self.active_rasterizer = Some(self.rasterizers.len());
        }

        self.rasterizers.push((key, Box::new(rasterizer)));
    }

    pub fn add_scene(&mut self, key: Key, scene: Scene) {
        if self.active_scene.is_none() {
            self.active_scene = Some(self.scenes.len());
        }

        self.scenes.push((key, scene));
    }

    fn update_frame(&mut self) {
        match (self.active_rasterizer, self.active_scene) {
            (Some(rasterizer_id), Some(scene_id)) => {
                let rasterizer = &mut self.rasterizers[rasterizer_id].1;
                let scene = &mut self.scenes[scene_id].1;

                self.framebuffer.reset();
                scene(&mut **rasterizer, &mut self.framebuffer);
                self.frame.reconstruct(ReconstructionFilter::Box, &mut self.framebuffer);
            }
            _ => (),
        }
    }

    pub fn run(&mut self) {
        // first frame!
        self.update_frame();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window.get_keys_pressed(minifb::KeyRepeat::No).map(|keys| {
                for k in keys {
                    for (i, (key, _)) in self.rasterizers.iter().enumerate() {
                        if *key == k {
                            self.active_rasterizer = Some(i);
                        }
                    }
                    for (i, (key, _)) in self.scenes.iter().enumerate() {
                        if *key == k {
                            self.active_scene = Some(i);
                        }
                    }
                }

                self.update_frame();
            });

            self.window
                .update_with_buffer(&self.frame.data, self.width as _, self.height as _)
                .unwrap();
        }
    }
}
