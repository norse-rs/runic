mod filter;
mod frame;
mod math;
mod paths;
mod polynomial;
mod rasterizer;
mod rect;
mod sample;

pub use crate::filter::*;
pub use crate::frame::*;
pub use crate::paths::*;
pub use crate::polynomial::*;
pub use crate::rasterizer::*;
pub use crate::rect::*;
pub use crate::sample::*;
pub use minifb::*;

pub type Scene = fn(&mut dyn Rasterizer, &mut Framebuffer);

pub struct App {
    width: u32,
    height: u32,
    frame: Frame,
    framebuffer: Framebuffer,
    window: Window,
    colorspace: Colorspace,

    rasterizers: Vec<(Key, Box<dyn Rasterizer>, UniformSampler)>,
    scenes: Vec<(Key, Scene)>,
    filters: Vec<(Key, Box<dyn Filter>)>,

    active_rasterizer: Option<usize>,
    active_scene: Option<usize>,
    active_filter: Option<usize>,
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

        window.limit_update_rate(Some(std::time::Duration::from_micros(10000)));

        App {
            width,
            height,
            frame,
            framebuffer,
            window,
            colorspace: Colorspace::Srgb,
            rasterizers: Vec::new(),
            active_rasterizer: None,
            scenes: Vec::new(),
            active_scene: None,
            filters: Vec::new(),
            active_filter: None,
        }
    }

    pub fn add_rasterizer<R: Rasterizer + 'static>(&mut self, key: Key, rasterizer: R, sampler: UniformSampler) {
        if self.active_rasterizer.is_none() {
            self.active_rasterizer = Some(self.rasterizers.len());
        }

        self.rasterizers.push((key, Box::new(rasterizer), sampler));
    }

    pub fn add_scene(&mut self, key: Key, scene: Scene) {
        if self.active_scene.is_none() {
            self.active_scene = Some(self.scenes.len());
        }

        self.scenes.push((key, scene));
    }

    pub fn add_filter<F: Filter + 'static>(&mut self, key: Key, filter: F) {
        if self.active_filter.is_none() {
            self.active_filter = Some(self.filters.len());
        }

        self.filters.push((key, Box::new(filter)));
    }

    fn update_frame(&mut self) {
        match (self.active_rasterizer, self.active_scene, self.active_filter) {
            (Some(rasterizer_id), Some(scene_id), Some(filter_id)) => {
                let (_, rasterizer, sampler) = &mut self.rasterizers[rasterizer_id];
                let scene = &mut self.scenes[scene_id].1;
                let filter = &self.filters[filter_id].1;

                let start = std::time::Instant::now();
                self.framebuffer.reset();
                sampler.populate(&mut self.framebuffer);

                print!("render scene..");
                scene(&mut **rasterizer, &mut self.framebuffer);
                println!("{:?}", start.elapsed());

                print!("reconstruct frame..");
                self.frame
                    .reconstruct(&mut self.framebuffer, &**filter, self.colorspace);
                println!("{:?}", start.elapsed());

                self.window.set_title(&format!("{} - Scene {}", rasterizer.name(), scene_id));
            }
            _ => (),
        }
    }

    pub fn run(&mut self) {
        // first frame!
        self.update_frame();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .get_keys_pressed(minifb::KeyRepeat::No)
                .map(|keys| {
                    let mut update_frame = false;

                    if keys.is_empty() {
                        return;
                    }

                    for k in keys {
                        for (i, (key, _, _)) in self.rasterizers.iter().enumerate() {
                            if *key == k {
                                self.active_rasterizer = Some(i);
                                update_frame = true;
                            }
                        }
                        for (i, (key, _)) in self.scenes.iter().enumerate() {
                            if *key == k {
                                self.active_scene = Some(i);
                                update_frame = true;
                            }
                        }
                        for (i, (key, _)) in self.filters.iter().enumerate() {
                            if *key == k {
                                self.active_filter = Some(i);
                                update_frame = true;
                            }
                        }

                        // Toggle colorspace
                        match k {
                            Key::S => {
                                self.colorspace = match self.colorspace {
                                    Colorspace::Linear => Colorspace::Srgb,
                                    Colorspace::Srgb => Colorspace::Linear,
                                };
                                update_frame = true;
                            },
                            Key::P => {
                                if let Some(pos) = self.window.get_mouse_pos(minifb::MouseMode::Discard) {
                                    let y = pos.1 as usize;
                                    let x = pos.0 as usize;
                                    println!("pos: {:?} {:?}", pos, (self.frame.data[y * self.width as usize + x] & 0xFF));
                                }
                            },
                            _ => {}
                        }
                    }

                    if update_frame {
                        self.update_frame();
                    }
                });

            self.window
                .update_with_buffer(&self.frame.data, self.width as _, self.height as _)
                .unwrap();
        }
    }
}
