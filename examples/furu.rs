use glyph_brush_layout::{GlyphPositioner, SectionGeometry, SectionText};
use runic::Rasterizer;
use rusttype::{Font, Scale, Segment};

const WIDTH: u32 = 480;
const HEIGHT: u32 = 260;

const ROBOTO: &[u8] = include_bytes!("../assets/Roboto-Regular.ttf");

fn main() {
    let mut app = runic::App::new(WIDTH, HEIGHT, runic::Scale::X2);

    app.add_rasterizer(
        runic::Key::F1,
        runic::CoarseRasterizer {
            direction: runic::CoarseDirection::Y,
            filter: runic::BoxFilter::new(-0.5, 0.5),
        },
        runic::UniformSampler { nx: 1, ny: 1 },
    );
    app.add_rasterizer(
        runic::Key::F2,
        runic::CoarseRasterizer {
            direction: runic::CoarseDirection::Y,
            filter: runic::BoxFilter::new(-0.5, 0.5),
        },
        runic::UniformSampler { nx: 1, ny: 1 },
    );
    app.add_rasterizer(
        runic::Key::F3,
        runic::CoarseRasterizer {
            direction: runic::CoarseDirection::XY,
            filter: runic::Smoothstep { e0: -0.5, e1: 0.5 },
        },
        runic::UniformSampler { nx: 1, ny: 1 },
    );
    app.add_rasterizer(
        runic::Key::F4,
        runic::HatiRasterizer {
            filter: runic::Smoothstep { e0: -0.5, e1: 0.5 },
        },
        runic::UniformSampler { nx: 1, ny: 1 },
    );

    // app.add_rasterizer(runic::Key::F4, runic::DistanceRasterizer { filter: runic::BoxFilter::new(-0.7, 0.7) }, runic::UniformSampler { nx: 1, ny: 1 });
    // app.add_rasterizer(runic::Key::F3, runic::DistanceRasterizer { filter: runic::RadialBoxFilter { radius: 0.7 } }, runic::UniformSampler { nx: 1, ny: 1 });
    // app.add_rasterizer(runic::Key::F4, runic::CoarseRasterizer { filter: runic::StepFilter }, runic::UniformSampler { nx: 1, ny: 1 });
    // app.add_rasterizer(runic::Key::F5, runic::CoarseRasterizer { filter: runic::StepFilter }, runic::UniformSampler { nx: 8, ny: 8 });
    app.add_rasterizer(
        runic::Key::F6,
        runic::AnalyticBoxRasterizer,
        runic::UniformSampler { nx: 1, ny: 1 },
    );
    app.add_rasterizer(
        runic::Key::F7,
        runic::DistanceRasterizer {
            filter: runic::Smoothstep { e0: -0.7, e1: 0.7 },
        },
        runic::UniformSampler { nx: 1, ny: 1 },
    );
    app.add_rasterizer(
        runic::Key::F8,
        runic::GouacheRasterizer {
            filter: runic::Smoothstep { e0: -0.5, e1: 0.5 },
        },
        runic::UniformSampler { nx: 1, ny: 1 },
    );

    app.add_scene(runic::Key::Key1, render_scene0);
    app.add_scene(runic::Key::Key2, render_scene1);

    app.add_scene(runic::Key::Key3, render_scene2);
    app.add_scene(runic::Key::Key4, render_scene3);
    app.add_scene(runic::Key::Key5, render_scene4);

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
        .move_to(glam::vec2(60.2, 80.4))
        .line_to(glam::vec2(80.0, 80.4))
        .line_to(glam::vec2(89.0, 8.4))
        .line_to(glam::vec2(70.2, 8.0))
        .close()
        .finish()];
    let aabb_triangle1 = runic::Aabb::from_segments(&segments_triangle1);

    // rasterize scene
    let path_triangle0 = rasterizer.create_path(&segments_triangle0);
    let path_triangle1 = rasterizer.create_path(&segments_triangle1);

    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(200.0, 0.0),
            extent_local: glam::vec2(100.0, 100.0),
            offset_curve: glam::vec2(0.0, 0.0),
            extent_curve: glam::vec2(100.0, 100.0),
        },
        &path_triangle1,
    );

    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(0.0, 0.0),
            extent_local: glam::vec2(100.0, 100.0),
            offset_curve: glam::vec2(0.0, 0.0),
            extent_curve: glam::vec2(100.0, 100.0),
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

    let segments_quad0 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(0.0, 0.0))
        .quad_to(glam::vec2(20.0, 70.0), glam::vec2(100.0, 100.0))
        .finish()];
    let aabb_quad0 = runic::Aabb::from_segments(&segments_quad0);

    let segments_quad1 = vec![runic::PathBuilder::new()
        .move_to(glam::vec2(0.0, 0.0))
        .quad_to(glam::vec2(100.0, 50.0), glam::vec2(0.0, 100.0))
        .monotonize()
        .finish()];
    dbg!(&segments_quad1);
    let aabb_quad1 = runic::Aabb::from_segments(&segments_quad1);

    // rasterize scene
    let path_line0 = rasterizer.create_path(&segments_line0);
    let path_line1 = rasterizer.create_path(&segments_line1);
    let path_line2 = rasterizer.create_path(&segments_line2);
    let path_line3 = rasterizer.create_path(&segments_line3);
    let path_quad0 = rasterizer.create_path(&segments_quad0);
    let path_quad1 = rasterizer.create_path(&segments_quad1);

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

    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(240.0, 20.0),
            extent_local: glam::vec2(100.0, 100.0),
            offset_curve: aabb_quad0.min,
            extent_curve: aabb_quad0.max - aabb_quad0.min,
        },
        &path_quad0,
    );
    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(360.0, 20.0),
            extent_local: glam::vec2(100.0, 100.0),
            offset_curve: glam::vec2(0.0, 0.0),
            extent_curve: glam::vec2(100.0, 100.0),
        },
        &path_quad1,
    );
}

fn render_scene2(rasterizer: &mut dyn Rasterizer, framebuffer: &mut runic::Framebuffer) {
    rasterizer.cmd_fill(
        framebuffer,
        glam::Vec2::new(40.0, 10.0),
        glam::Vec2::new(320.0, 40.0),
        0.5,
    );
    let num_bands = 100;
    for i in 0..num_bands {
        rasterizer.cmd_fill(
            framebuffer,
            glam::Vec2::new(50.0 + i as f32 * 3.0, 20.0),
            glam::Vec2::new(3.0, 20.0),
            (i + 1) as f32 / num_bands as f32,
        );
    }
}

fn render_scene3(rasterizer: &mut dyn Rasterizer, framebuffer: &mut runic::Framebuffer) {
    let radius = 100.0;
    let center = glam::vec2(0.0, 0.0);

    let num_segments = 32;
    let segment_step = 2.0 * std::f32::consts::PI / num_segments as f32;

    let mut path = runic::PathBuilder::new();

    for n in 0..num_segments {
        let (s0, c0) = (segment_step * (n as f32 + 0.25)).sin_cos();
        let (s1, c1) = (segment_step * (n as f32 - 0.25)).sin_cos();

        path = path
            .move_to(center)
            .line_to(glam::vec2(c0, s0) * radius + center)
            .line_to(glam::vec2(c1, s1) * radius + center)
            .close();
    }

    let segments = vec![path.finish()];
    let aabb = runic::Aabb::from_segments(&segments);

    // rasterize scene
    let path = rasterizer.create_path(&segments);

    rasterizer.cmd_draw(
        framebuffer,
        runic::Rect {
            offset_local: glam::vec2(20.0, 20.0),
            extent_local: glam::vec2(200.0, 200.0),
            offset_curve: glam::vec2(-radius, -radius),
            extent_curve: glam::vec2(2.0 * radius, 2.0 * radius),
        },
        &path,
    );
}

fn render_scene4(rasterizer: &mut dyn Rasterizer, framebuffer: &mut runic::Framebuffer) {
    let mut offset_y = 0.0;
    for size in &[18.0, 24.0, 32.0, 48.0, 72.0] {
        let font = Font::from_bytes(&ROBOTO).unwrap();
        let glyphs = glyph_brush_layout::Layout::default().calculate_glyphs(
            &[font],
            &SectionGeometry {
                screen_position: (10.0, offset_y),
                ..SectionGeometry::default()
            },
            &[SectionText {
                text: "quick brown fox",
                scale: Scale::uniform(*size),
                ..SectionText::default()
            }],
        );

        offset_y += size;

        for (glyph, _, font) in glyphs {
            let mut path = runic::PathBuilder::new();

            let bbox = glyph.unpositioned().exact_bounding_box().unwrap();
            let shapes = glyph.unpositioned().shape().unwrap();
            let mut pos = glyph.position();
            pos.y -= bbox.max.y + bbox.min.y;

            for shape in shapes {
                for segment in &shape.segments {
                    match segment {
                        Segment::Line(line) => {
                            path = path.move_to(glam::vec2(
                                line.p[0].x - bbox.min.x,
                                line.p[0].y + bbox.max.y,
                            ));
                            path = path.line_to(glam::vec2(
                                line.p[1].x - bbox.min.x,
                                line.p[1].y + bbox.max.y,
                            ));
                        }
                        Segment::Curve(curve) => {
                            path = path.move_to(glam::vec2(
                                curve.p[0].x - bbox.min.x,
                                curve.p[0].y + bbox.max.y,
                            ));
                            path = path.quad_to(
                                glam::vec2(curve.p[1].x - bbox.min.x, curve.p[1].y + bbox.max.y),
                                glam::vec2(curve.p[2].x - bbox.min.x, curve.p[2].y + bbox.max.y),
                            );
                        }
                    }
                }
            }

            path = path.monotonize();
            let mut curves = path.finish();

            let rect = runic::Rect {
                offset_local: glam::vec2(pos.x + bbox.min.x as f32, pos.y + bbox.min.y as f32),
                extent_local: glam::vec2(
                    bbox.max.x as f32 - bbox.min.x as f32,
                    bbox.max.y as f32 - bbox.min.y as f32,
                ),
                offset_curve: glam::vec2(0.0, 0.0),
                extent_curve: glam::vec2(
                    bbox.max.x as f32 - bbox.min.x as f32,
                    bbox.max.y as f32 - bbox.min.y as f32,
                ),
            };

            rasterizer.cmd_draw(framebuffer, rect, &curves);
        }
    }
}
