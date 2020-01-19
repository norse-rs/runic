
mod box_filter;
mod bounds;
mod step;
mod lanzcos;
mod tent;
mod radial_box;
mod smoothstep;

pub use self::box_filter::*;
pub use self::step::*;
pub use self::bounds::*;
pub use self::lanzcos::*;
pub use self::tent::*;
pub use self::radial_box::*;
pub use self::smoothstep::*;

pub trait Filter {
    fn name(&self) -> String;

    // Probability distribution function.
    fn pdf(&self, x: f32) -> f32;

    // Cumulative distribution function.
    fn cdf(&self, x: f32) -> f32;

    fn relative_bounds(&self, pos: (f32, f32)) -> RelativeBounds;
}
