use minifb::{Key, Window, WindowOptions};
use runic::Rasterizer;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 360;

fn main() {
    let mut frame = runic::Frame::new(WIDTH, HEIGHT);
    let mut framebuffer = runic::Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "runic - furu",
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

    let segments_triangle = vec![vec![
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
    let aabb_triangle = runic::Aabb::from_segments(&segments_triangle);

    framebuffer.add_samples_pos(glam::Vec2::new(0.5, 0.5));

    {
        let mut rasterizer = runic::DistanceRasterizer {};
        let path_triangle = rasterizer.create_path(&segments_triangle);

        for i in 0..20 {
            rasterizer.cmd_fill(
                (0, &mut framebuffer),
                glam::Vec2::new(50.0 + i as f32 * 25.0, 50.0),
                glam::Vec2::new(25.0, 50.0),
                (i + 1) as f32 * 0.05,
            );
        }
        rasterizer.cmd_draw(
            (0, &mut framebuffer),
            runic::Rect {
                offset_local: glam::vec2(50.0, 120.0),
                extent_local: glam::vec2(100.0, 200.0),
                offset_curve: aabb_triangle.min,
                extent_curve: aabb_triangle.max - aabb_triangle.min,
            },
            &path_triangle,
        );
    }

    frame.reconstruct(runic::ReconstructionFilter::Box, &mut framebuffer);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&frame.data, WIDTH as _, HEIGHT as _)
            .unwrap();
    }
}
