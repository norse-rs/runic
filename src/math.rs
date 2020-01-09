pub fn line_eval(p0: f32, p1: f32, t: f32) -> f32 {
    (1.0 - t) * p0 + t * p1
}

pub fn line_raycast(p0: f32, p1: f32, p: f32) -> f32 {
    (p - p0) / (p1 - p0)
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