pub fn line_eval(p0: f32, p1: f32, t: f32) -> f32 {
    (1.0 - t) * p0 + t * p1
}

pub fn line_raycast(p0: f32, p1: f32, p: f32) -> f32 {
    (p - p0) / (p1 - p0)
}

pub fn quad_eval(p0: f32, p1: f32, p2: f32, t: f32) -> f32 {
    (1.0 - t) * (1.0 - t) * p0 + 2.0 * t * (1.0 - t) * p1 + t * t * p2
}

pub fn quad_raycast(p0: f32, p1: f32, p2: f32, t: f32) -> f32 {
    if t <= p0.min(p2) || t >= p0.max(p2) {
        return line_raycast(p0, p2, t);
    }

    let sign = (p2 > t) as i32 - (p0 > t) as i32;

    let a = p0 - 2.0 * p1 + p2;
    let b = p0 - p1;
    let c = p0 - t;

    let dscr_sq = b * b - a * c;
    if a.abs() < 0.0001 {
        line_raycast(p0, p2, t)
    } else {
        (b + sign as f32 * dscr_sq.sqrt()) / a
    }
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    x.min(max).max(min)
}

pub fn linear_to_srgb(value: f32) -> f32 {
    if value < 0.0031308 {
        value * 12.92
    } else {
        1.055 * value.powf(5.0 / 12.0) - 0.055
    }
}