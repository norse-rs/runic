use crate::{
    math::*, rasterize_each_with_bias, BoxFilter, Curve, Framebuffer, Rasterizer, Rect, SampleId,
    Segment,
};

pub struct AnalyticBoxRasterizer;

impl AnalyticBoxRasterizer {
    fn quad_line_coverage_right(
        p0: glam::Vec2,
        p1: glam::Vec2,
        p2: glam::Vec2,
        xx0: f32,
        xx1: f32,
        dy: f32,
    ) -> f32 {
        let yy0 = 0.0;
        let yy1 = dy;

        let tx0 = quad_raycast(p0.x(), p1.x(), p2.x(), xx0); // raycast y direction
        let tx1 = quad_raycast(p0.x(), p1.x(), p2.x(), xx1); // raycast y direction

        let ty0 = quad_raycast(p0.y(), p1.y(), p2.y(), yy0); // raycast x direction
        let ty1 = quad_raycast(p0.y(), p1.y(), p2.y(), yy1); // raycast x direction

        let t0 = clamp(ty0, tx0, tx1);
        let t1 = clamp(ty1, tx0, tx1);

        let x0 = quad_eval(p0.x(), p1.x(), p2.x(), t0);
        let x1 = quad_eval(p0.x(), p1.x(), p2.x(), t1);

        let y0 = quad_eval(p0.y(), p1.y(), p2.y(), t0);
        let y1 = quad_eval(p0.y(), p1.y(), p2.y(), t1);

        let rectangle = clamp(clamp(y1, 0.0, 1.0) * (xx1 - x1), 0.0, 1.0);
        let trapezoid = clamp((y0 + y1) * 0.5 * (x1 - x0), 0.0, 1.0);

        let area = rectangle + trapezoid;
        clamp(area, 0.0, 1.0)
    }

    fn quad_line_coverage_left(
        p0: glam::Vec2,
        p1: glam::Vec2,
        p2: glam::Vec2,
        xx0: f32,
        xx1: f32,
        dy: f32,
    ) -> f32 {
        let yy0 = 0.0;
        let yy1 = dy;

        let tx0 = quad_raycast(p0.x(), p1.x(), p2.x(), xx0); // raycast y direction
        let tx1 = quad_raycast(p0.x(), p1.x(), p2.x(), xx1); // raycast y direction

        let ty0 = quad_raycast(p0.y(), p1.y(), p2.y(), yy0); // raycast x direction
        let ty1 = quad_raycast(p0.y(), p1.y(), p2.y(), yy1); // raycast x direction

        let t0 = clamp(ty0, tx0, tx1);
        let t1 = clamp(ty1, tx0, tx1);

        let x0 = quad_eval(p0.x(), p1.x(), p2.x(), t0);
        let x1 = quad_eval(p0.x(), p1.x(), p2.x(), t1);

        let y0 = quad_eval(p0.y(), p1.y(), p2.y(), t0);
        let y1 = quad_eval(p0.y(), p1.y(), p2.y(), t1);

        let rectangle = clamp(clamp(y1, 0.0, 1.0) * (x1 - xx1), 0.0, 1.0);
        let trapezoid = clamp((y0 + y1) * 0.5 * (x0 - x1), 0.0, 1.0);

        let area = rectangle + trapezoid;
        clamp(area, 0.0, 1.0)
    }

    fn line_coverage_right(p0: glam::Vec2, p1: glam::Vec2, xx0: f32, xx1: f32, dy: f32) -> f32 {
        let yy0 = 0.0;
        let yy1 = dy;

        let tx0 = line_raycast(p0.x(), p1.x(), xx0); // raycast y direction
        let tx1 = line_raycast(p0.x(), p1.x(), xx1); // raycast y direction

        let ty0 = line_raycast(p0.y(), p1.y(), yy0); // raycast x direction
        let ty1 = line_raycast(p0.y(), p1.y(), yy1); // raycast x direction

        let t0 = clamp(ty0, tx0, tx1);
        let t1 = clamp(ty1, tx0, tx1);

        let x0 = line_eval(p0.x(), p1.x(), t0);
        let x1 = line_eval(p0.x(), p1.x(), t1);

        let y0 = line_eval(p0.y(), p1.y(), t0);
        let y1 = line_eval(p0.y(), p1.y(), t1);

        let rectangle = clamp(clamp(y1, 0.0, 1.0) * (xx1 - x1), 0.0, 1.0);
        let trapezoid = clamp((y0 + y1) * 0.5 * (x1 - x0), 0.0, 1.0);

        let area = rectangle + trapezoid;
        clamp(area, 0.0, 1.0)
    }

    fn line_coverage_left(p0: glam::Vec2, p1: glam::Vec2, xx0: f32, xx1: f32, dy: f32) -> f32 {
        let yy0 = 0.0;
        let yy1 = dy;

        let tx0 = line_raycast(p0.x(), p1.x(), xx0); // raycast y direction
        let tx1 = line_raycast(p0.x(), p1.x(), xx1); // raycast y direction

        let ty0 = line_raycast(p0.y(), p1.y(), yy0); // raycast x direction
        let ty1 = line_raycast(p0.y(), p1.y(), yy1); // raycast x direction

        let t0 = clamp(ty0, tx0, tx1);
        let t1 = clamp(ty1, tx0, tx1);

        let x0 = line_eval(p0.x(), p1.x(), t0);
        let x1 = line_eval(p0.x(), p1.x(), t1);

        let y0 = line_eval(p0.y(), p1.y(), t0);
        let y1 = line_eval(p0.y(), p1.y(), t1);

        let rectangle = clamp(clamp(y1, 0.0, 1.0) * (x1 - xx1), 0.0, 1.0);
        let trapezoid = clamp((y0 + y1) * 0.5 * (x0 - x1), 0.0, 1.0);

        let area = rectangle + trapezoid;
        clamp(area, 0.0, 1.0)
    }
}

impl Rasterizer for AnalyticBoxRasterizer {
    fn name(&self) -> String {
        format!("AnalyticBoxRasterizer")
    }

    fn create_path(&mut self, segments: &[Segment]) -> Vec<Curve> {
        let mut curves = Vec::new();
        for segment in segments {
            for curve in segment {
                curves.push(*curve);
            }
        }
        curves
    }

    fn cmd_draw(&mut self, framebuffer: &mut Framebuffer, rect: Rect, path: &[Curve]) {
        let filter = BoxFilter::new(-0.5, 0.5);

        rasterize_each_with_bias((1.0, 1.0), framebuffer, rect, |pos_curve, dxdy| {
            let mut coverage = 0.0;

            // assert_eq!(dxdy, glam::vec2(1.0, 1.0)); // TODO

            for curve in path {
                match curve {
                    Curve::Line { p0, p1 } => {
                        let mut p0 = *p0;
                        let mut p1 = *p1;

                        let sign_x = (p1.x() > p0.x()) as i32 - (p0.x() > p1.x()) as i32;
                        let sign_y = (p1.y() > p0.y()) as i32 - (p0.y() > p1.y()) as i32;

                        let x_min = p1.x().min(p0.x());
                        let x_max = p1.x().max(p0.x());
                        let hit = ((x_max > pos_curve.x() - 0.5 * dxdy.x())
                            && (x_min < pos_curve.x() + 0.5 * dxdy.x()));

                        p0 -= glam::vec2(0.0, pos_curve.y() - 0.5);
                        p1 -= glam::vec2(0.0, pos_curve.y() - 0.5);

                        if hit {
                            if sign_x > 0 {
                                if sign_y > 0 {
                                    let xx0 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p0.x(), p1.x());
                                    let xx1 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p0.x(), p1.x());
                                    coverage +=
                                        Self::line_coverage_right(p0, p1, xx0, xx1, dxdy.y());
                                } else {
                                    let xx0 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p0.x(), p1.x());
                                    let xx1 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p0.x(), p1.x());
                                    coverage +=
                                        Self::line_coverage_left(p1, p0, xx0, xx1, dxdy.y());
                                }
                            } else if sign_x < 0 {
                                if sign_y > 0 {
                                    let xx0 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p1.x(), p0.x());
                                    let xx1 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p1.x(), p0.x());
                                    coverage -=
                                        Self::line_coverage_left(p0, p1, xx0, xx1, dxdy.y());
                                } else {
                                    let xx0 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p1.x(), p0.x());
                                    let xx1 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p1.x(), p0.x());
                                    coverage -=
                                        Self::line_coverage_right(p1, p0, xx0, xx1, dxdy.y());
                                }
                            }
                        }
                    }
                    Curve::Quad { p0, p1, p2 } => {
                        let mut p0 = *p0;
                        let mut p1 = *p1;
                        let mut p2 = *p2;

                        let sign_x = (p2.x() > p0.x()) as i32 - (p0.x() > p2.x()) as i32;
                        let sign_y = (p2.y() > p0.y()) as i32 - (p0.y() > p2.y()) as i32;

                        let x_min = p2.x().min(p0.x());
                        let x_max = p2.x().max(p0.x());
                        let hit = ((x_max > pos_curve.x() - 0.5 * dxdy.x())
                            && (x_min < pos_curve.x() + 0.5 * dxdy.x()));

                        p0 -= glam::vec2(0.0, pos_curve.y() - 0.5);
                        p1 -= glam::vec2(0.0, pos_curve.y() - 0.5);
                        p2 -= glam::vec2(0.0, pos_curve.y() - 0.5);

                        if hit {
                            if sign_x > 0 {
                                if sign_y > 0 {
                                    let xx0 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p0.x(), p2.x());
                                    let xx1 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p0.x(), p2.x());
                                    coverage += Self::quad_line_coverage_right(
                                        p0,
                                        p1,
                                        p2,
                                        xx0,
                                        xx1,
                                        dxdy.y(),
                                    );
                                } else {
                                    let xx0 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p0.x(), p2.x());
                                    let xx1 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p0.x(), p2.x());
                                    coverage += Self::quad_line_coverage_left(
                                        p2,
                                        p1,
                                        p0,
                                        xx0,
                                        xx1,
                                        dxdy.y(),
                                    );
                                }
                            } else if sign_x < 0 {
                                if sign_y > 0 {
                                    let xx0 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p2.x(), p0.x());
                                    let xx1 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p2.x(), p0.x());
                                    coverage -= Self::quad_line_coverage_left(
                                        p0,
                                        p1,
                                        p2,
                                        xx0,
                                        xx1,
                                        dxdy.y(),
                                    );
                                } else {
                                    let xx0 = clamp(pos_curve.x() - 0.5 * dxdy.x(), p2.x(), p0.x());
                                    let xx1 = clamp(pos_curve.x() + 0.5 * dxdy.x(), p2.x(), p0.x());
                                    coverage -= Self::quad_line_coverage_right(
                                        p2,
                                        p1,
                                        p0,
                                        xx0,
                                        xx1,
                                        dxdy.y(),
                                    );
                                }
                            }
                        }
                    }
                }
            }

            coverage
        });
    }
}
