use runic::{Rasterizer, BoxFilter, LanzcosFilter};

const WIDTH: u32 = 480;
const HEIGHT: u32 = 260;

fn main() {
    let mut app = runic::App::new(WIDTH, HEIGHT, runic::Scale::X2);

    app.add_rasterizer(runic::Key::F1, runic::CoarseRasterizer { filter: runic::BoxFilter::new(-0.5, 0.5) }, runic::UniformSampler { nx: 1, ny: 1 });
    app.add_rasterizer(runic::Key::F2, runic::DistanceRasterizer {}, runic::UniformSampler { nx: 1, ny: 1 });
    app.add_rasterizer(runic::Key::F3, runic::CoarseRasterizer { filter: runic::StepFilter }, runic::UniformSampler { nx: 1, ny: 1 });
    app.add_rasterizer(runic::Key::F4, runic::CoarseRasterizer { filter: runic::StepFilter }, runic::UniformSampler { nx: 8, ny: 8 });

    app.add_scene(runic::Key::Key1, render_scene0);

    app.add_filter(runic::Key::N, BoxFilter::new(-0.5, 0.5));

    app.run();
}

fn render_scene0(rasterizer: &mut dyn Rasterizer, framebuffer: &mut runic::Framebuffer) {
    // scene geometry
    let segments_triangle0 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(0.0, 0.0))
        .line_to(glam::vec2(25.0, 100.0))
        .line_to(glam::vec2(50.0, 80.0))
        .close()
        .finish()];
    let aabb_triangle0 = runic::Aabb::from_segments(&segments_triangle0);

    let segments_triangle1 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(25.0, 0.0))
        .line_to(glam::vec2(0.0, 100.0))
        .line_to(glam::vec2(50.0, 100.0))
        .close()
        .finish()];
    let aabb_triangle1 = runic::Aabb::from_segments(&segments_triangle1);

    // rasterize scene
    let path_triangle0 = rasterizer.create_path(&segments_triangle0);
    let path_triangle1 = rasterizer.create_path(&segments_triangle1);

    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(10.0, 10.0),
            extent_local: glam::vec2(100.0, 200.0),
            offset_curve: aabb_triangle0.min,
            extent_curve: aabb_triangle0.max - aabb_triangle0.min,
        },
        &path_triangle0,
    );
    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(230.0, 10.0),
            extent_local: glam::vec2(100.0, 200.0),
            offset_curve: aabb_triangle1.min,
            extent_curve: aabb_triangle1.max - aabb_triangle1.min,
        },
        &path_triangle1,
    );
}
