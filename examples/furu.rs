use runic::{Rasterizer};

const WIDTH: u32 = 480;
const HEIGHT: u32 = 260;

fn main() {
    let mut app = runic::App::new(WIDTH, HEIGHT, runic::Scale::X1);

    app.add_rasterizer(runic::Key::F1, runic::CoarseRasterizer { filter: runic::BoxFilter::new(-0.5, 0.5) }, runic::UniformSampler { nx: 1, ny: 1 });
    app.add_rasterizer(runic::Key::F2, runic::DistanceRasterizer {}, runic::UniformSampler { nx: 1, ny: 1 });
    app.add_rasterizer(runic::Key::F3, runic::CoarseRasterizer { filter: runic::StepFilter }, runic::UniformSampler { nx: 1, ny: 1 });
    app.add_rasterizer(runic::Key::F4, runic::CoarseRasterizer { filter: runic::StepFilter }, runic::UniformSampler { nx: 8, ny: 8 });
    app.add_rasterizer(runic::Key::F5, runic::AnalyticBoxRasterizer, runic::UniformSampler { nx: 1, ny: 1 });

    app.add_scene(runic::Key::Key1, render_scene0);
    app.add_scene(runic::Key::Key2, render_scene1);
    app.add_scene(runic::Key::Key3, render_scene2);

    app.add_filter(runic::Key::N, runic::BoxFilter::new(-0.5, 0.5));
    app.add_filter(runic::Key::B, runic::TentFilter);

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
            extent_local: glam::vec2(50.0, 100.0),
            offset_curve: aabb_triangle0.min,
            extent_curve: aabb_triangle0.max - aabb_triangle0.min,
        },
        &path_triangle0,
    );
    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(80.0, 10.0),
            extent_local: glam::vec2(50.0, 100.0),
            offset_curve: aabb_triangle1.min,
            extent_curve: aabb_triangle1.max - aabb_triangle1.min,
        },
        &path_triangle1,
    );
}

fn render_scene1(rasterizer: &mut dyn Rasterizer, framebuffer: &mut runic::Framebuffer) {
    // scene geometry
    let segments_line0 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(100.0, 0.0))
        .line_to(glam::vec2(0.0, 25.0))
        .finish()];
    let aabb_line0 = runic::Aabb::from_segments(&segments_line0);

    let segments_line1 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(0.0, 0.0))
        .line_to(glam::vec2(100.0, 25.0))
        .finish()];
    let aabb_line1 = runic::Aabb::from_segments(&segments_line1);

    let segments_line2 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(0.0, 25.0))
        .line_to(glam::vec2(100.0, 0.0))
        .finish()];
    let aabb_line2 = runic::Aabb::from_segments(&segments_line2);

    let segments_line3 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(100.0, 25.0))
        .line_to(glam::vec2(0.0, 0.0))
        .finish()];
    let aabb_line3 = runic::Aabb::from_segments(&segments_line3);

    // rasterize scene
    let path_line0 = rasterizer.create_path(&segments_line0);
    let path_line1 = rasterizer.create_path(&segments_line1);
    let path_line2 = rasterizer.create_path(&segments_line2);
    let path_line3 = rasterizer.create_path(&segments_line3);

    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(10.0, 20.0),
            extent_local: glam::vec2(100.0, 25.0),
            offset_curve: dbg!(aabb_line0.min),
            extent_curve: dbg!(aabb_line0.max - aabb_line0.min),
        },
        &path_line0,
    );
    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(120.0, 20.0),
            extent_local: glam::vec2(100.0, 25.0),
            offset_curve: aabb_line1.min,
            extent_curve: aabb_line1.max - aabb_line1.min,
        },
        &path_line1,
    );
    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(120.0, 50.0),
            extent_local: glam::vec2(100.0, 25.0),
            offset_curve: aabb_line2.min,
            extent_curve: aabb_line2.max - aabb_line2.min,
        },
        &path_line2,
    );
    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(10.0, 50.0),
            extent_local: glam::vec2(100.0, 25.0),
            offset_curve: aabb_line3.min,
            extent_curve: aabb_line3.max - aabb_line3.min,
        },
        &path_line3,
    );
}

fn render_scene2(rasterizer: &mut dyn Rasterizer, framebuffer: &mut runic::Framebuffer) {
    rasterizer.cmd_fill(
        framebuffer,
        glam::Vec2::new(40.0, 10.0),
        glam::Vec2::new(320.0, 40.0),
        0.5,
    );
    for i in 0..100 {
        rasterizer.cmd_fill(
            framebuffer,
            glam::Vec2::new(50.0 + i as f32 * 3.0, 20.0),
            glam::Vec2::new(3.0, 20.0),
            (i + 1) as f32 * 0.01,
        );
    }
}
