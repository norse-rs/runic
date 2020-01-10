use runic::{Rasterizer, BoxFilter, LanzcosFilter, TentFilter};

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

fn main() {
    let mut app = runic::App::new(WIDTH, HEIGHT, runic::Scale::X1);

    app.add_rasterizer(runic::Key::F1, runic::CoarseRasterizer { filter: runic::StepFilter }, runic::UniformSampler { nx: 32, ny: 32 });

    app.add_scene(runic::Key::Key1, render_scene0);

    app.add_filter(runic::Key::N, BoxFilter::new(-0.5, 0.5));
    app.add_filter(runic::Key::M, LanzcosFilter { a: 3.0 });
    app.add_filter(runic::Key::B, TentFilter);

    app.run();
}

fn render_scene0(rasterizer: &mut dyn Rasterizer, framebuffer: &mut runic::Framebuffer) {
    // scene geometry
    let segments_triangle0 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(10.0, 0.0))
        .line_to(glam::vec2(20.0, 100.0))
        .line_to(glam::vec2(20.0, 0.0))
        .close()
        .finish()];
    let aabb_triangle0 = runic::Aabb::from_segments(&segments_triangle0);

    // rasterize scene
    let path_triangle0 = rasterizer.create_path(&segments_triangle0);

    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(4.0, 4.0),
            extent_local: glam::vec2(20.0, 100.0),
            offset_curve: aabb_triangle0.min,
            extent_curve: aabb_triangle0.max - aabb_triangle0.min,
        },
        &path_triangle0,
    );
}
