use crate::{
    rasterize_each_with_bias, Filter, Curve, Framebuffer, Rasterizer, Rect,
    Segment
};
use std::f32::consts::PI;

pub struct DistanceRasterizer<F> {
    pub filter: F,
}

// ---------------------
// Quadratic distance calculation based on: https://www.shadertoy.com/view/3tlSzH
/*
*	zlnimda wrote this file and is under license CC-BY-SA-4.0
* 	( see legal notice: https://creativecommons.org/licenses/by-sa/4.0/legalcode )
* 	Bezier algo from my previous shader: https://www.shadertoy.com/view/Mt33zr
* 	Points and distance view from iq: https://www.shadertoy.com/view/MlKcDD
*/
fn cardano(p: f32, q: f32) -> glam::Vec3 {
    let p3 = p * p * p;
    let d = -(4.0 * p3 + 27.0 * q * q);
    if d > 0.0 {
        let a = 2.0 * (-p / 3.0).sqrt();
        let b = ((27.0 / (-p3)).sqrt() * (-q / 2.0)).acos() / 3.0;
        glam::vec3(a * b.cos(), a * (b + 2.0 * PI / 3.0), a * (b + 4.0 * PI / 3.0))
    } else if d < 0.0 {
        let coeff = 1.0 / 3.0;
        let dd = (-d / 27.0).sqrt();
        let mut u = (-q + dd) / 2.0;
        u = u.signum() * u.abs().powf(coeff).abs();
        let mut v = (-q - dd) / 2.0;
        v = v.signum() * v.abs().powf(coeff);
        glam::vec3(u + v, u + v, u + v)
    } else {
        if p == 0.0 || q == 0.0 {
            glam::vec3(0.0, 0.0, 0.0)
        } else {
            let r = 3.0 * q / p;
            let r2 = -3.0 * q / (2.0 * p);
            glam::vec3(r, r2, r2)
        }
    }
}

fn distance_quadratic(p: glam::Vec2, p0: glam::Vec2, p1: glam::Vec2, p2: glam::Vec2) -> f32 {
    let a = p1 - p0;
    let b = p2 - p1 - a;

    let m = p0 - p;


    let t = {
        let (a, b, c, d) = (b.dot(b), 3.0 * a.dot(b), a.dot(a) * 2.0 + m.dot(b), m.dot(a));
        let unpress = b / (3.0 * a);
        let A = glam::vec3(b, c, d) / a;
        let p = A.y() - A.x()*A.x()/3.0;
        let q = A.x()*(2.0*A.x()*A.x()-9.0*A.y())/27.0+A.z();
        cardano(p, q) - glam::vec3(unpress, unpress, unpress)
    };

    let mut t_min = t.x().min(1.0).max(0.0);
    let mut n = p - Curve::Quad { p0, p1, p2 }.eval(t_min);
    let mut d = n.length();

    if t.y() >= 0.0 && t.y() <= 1.0 {
        let ny = p - Curve::Quad { p0, p1, p2 }.eval(t.y());
        if ny.length() < d {
            d = ny.length();
            t_min = t.y();
            n = ny;
        }
    }
    if t.z() >= 0.0 && t.z() <= 1.0 {
        let nz = p - Curve::Quad { p0, p1, p2 }.eval(t.z());
        if nz.length() < d {
            d = nz.length();
            t_min = t.z();
            n = nz;
        }
    }

    d * n.x().signum()
}
// ---------------------

impl<F: Filter> Rasterizer for DistanceRasterizer<F> {
    fn name(&self) -> String {
        format!("DistanceRasterizer :: {}", self.filter.name())
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

    fn cmd_draw(
        &mut self,
        framebuffer: &mut Framebuffer,
        rect: Rect,
        path: &[Curve],
    ) {
        rasterize_each_with_bias(
            (1.0, 1.0),
            framebuffer,
            rect,
            |pos_curve, dxdy| {
                let mut coverage = 0.0;
                let mut distance = 10000000.0f32;
                for curve in path {
                    match curve {
                        Curve::Line { p0, p1 } => {
                            let p0 = *p0 - pos_curve;
                            let p1 = *p1 - pos_curve;

                            let sign_y = (p1.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;

                            let dir = p1 - p0;
                            let dp = -p0;
                            let t = (dir.dot(dp) / dir.dot(dir)).min(1.0).max(0.0);
                            let n = (dp - dir * t) / dxdy;
                            let d = n.length() * n.x().signum();

                            coverage += sign_y as f32 * d.signum().min(0.0);
                            distance = distance.min(d.abs());
                        }
                        Curve::Quad { p0, p1, p2 } => {
                            let p0 = *p0 - pos_curve;
                            let p1 = *p1 - pos_curve;
                            let p2 = *p2 - pos_curve;

                            let sign_y = (p2.y() > 0.0) as i32 - (p0.y() > 0.0) as i32;
                            let d = distance_quadratic(glam::vec2(0.0, 0.0), p0, p1, p2);

                            coverage += sign_y as f32 * d.signum().min(0.0);
                            distance = distance.min(d.abs());
                        }
                    }
                }

                self.filter.cdf((2.0 * coverage - 1.0) * distance)
            },
        );
    }
}
