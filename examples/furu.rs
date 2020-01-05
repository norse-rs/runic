use minifb::{Key, Window, WindowOptions};
use runic::Rasterizer;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

fn render_scene0<R: Rasterizer>(rasterizer: &mut R, framebuffer: &mut runic::Framebuffer) {
    // scene geometry
    let segments_triangle0 = vec![vec![
        runic::Curve::Line {
            p0: glam::Vec2::new(0.0, 0.0),
            p1: glam::Vec2::new(25.0, 100.0),
        },
        runic::Curve::Line {
            p0: glam::Vec2::new(25.0, 100.0),
            p1: glam::Vec2::new(50.0, 80.0),
        },
        runic::Curve::Line {
            p0: glam::Vec2::new(50.0, 80.0),
            p1: glam::Vec2::new(0.0, 0.0),
        },
    ]];
    let aabb_triangle0 = runic::Aabb::from_segments(&segments_triangle0);

    let segments_triangle1 = vec![vec![
        runic::Curve::Line {
            p0: glam::Vec2::new(50.0, 0.0),
            p1: glam::Vec2::new(0.0, 80.0),
        },
        runic::Curve::Line {
            p0: glam::Vec2::new(0.0, 80.0),
            p1: glam::Vec2::new(25.0, 100.0),
        },
        runic::Curve::Line {
            p0: glam::Vec2::new(25.0, 100.0),
            p1: glam::Vec2::new(50.0, 0.0),
        },
    ]];
    let aabb_triangle1 = runic::Aabb::from_segments(&segments_triangle1);

    // prepare film
    framebuffer.reset();
    framebuffer.add_samples_pos(glam::Vec2::new(0.5, 0.5));

    // rasterize scene
    {
        let path_triangle0 = rasterizer.create_path(&segments_triangle0);
        let path_triangle1 = rasterizer.create_path(&segments_triangle1);

        for i in 0..20 {
            rasterizer.cmd_fill(
                (0, framebuffer),
                glam::Vec2::new(50.0 + i as f32 * 25.0, 50.0),
                glam::Vec2::new(25.0, 50.0),
                (i + 1) as f32 * 0.05,
            );
        }
        rasterizer.cmd_draw(
            (0, framebuffer),
            runic::Rect {
                offset_local: glam::vec2(50.0, 120.0),
                extent_local: glam::vec2(100.0, 200.0),
                offset_curve: aabb_triangle0.min,
                extent_curve: aabb_triangle0.max - aabb_triangle0.min,
            },
            &path_triangle0,
        );
        rasterizer.cmd_draw(
            (0, framebuffer),
            runic::Rect {
                offset_local: glam::vec2(300.0, 120.0),
                extent_local: glam::vec2(100.0, 200.0),
                offset_curve: aabb_triangle1.min,
                extent_curve: aabb_triangle1.max - aabb_triangle1.min,
            },
            &path_triangle1,
        );
    }
}

fn main() {
    let mut frame = runic::Frame::new(WIDTH, HEIGHT);
    let mut framebuffer = runic::Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "0 - CoarseRasterizer",
        WIDTH as _,
        HEIGHT as _,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: minifb::Scale::X1,
            scale_mode: minifb::ScaleMode::AspectRatioStretch,
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // Default
    let mut rasterizer = runic::CoarseRasterizer {};
    render_scene0(&mut rasterizer, &mut framebuffer);
    frame.reconstruct(runic::ReconstructionFilter::Box, &mut framebuffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.get_keys_pressed(minifb::KeyRepeat::No).map(|keys| {
            for k in keys {
                match k {
                    minifb::Key::F1 => {
                        window.set_title("0 - CoarseRasterizer");
                        let mut rasterizer = runic::CoarseRasterizer {};
                        render_scene0(&mut rasterizer, &mut framebuffer);
                    },
                    minifb::Key::F2 => {
                        window.set_title("0 - DistanceRasterizer");
                        let mut rasterizer = runic::DistanceRasterizer {};
                        render_scene0(&mut rasterizer, &mut framebuffer);
                    },
                    _ => (),
                }
            }

            frame.reconstruct(runic::ReconstructionFilter::Box, &mut framebuffer);
        });

        window
            .update_with_buffer(&frame.data, WIDTH as _, HEIGHT as _)
            .unwrap();
    }
}
