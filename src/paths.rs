pub type Segment = Vec<Curve>;

#[derive(Debug, Clone, Copy)]
pub enum Curve {
    Line {
        p0: glam::Vec2,
        p1: glam::Vec2,
    },
    Quad {
        p0: glam::Vec2,
        p1: glam::Vec2,
        p2: glam::Vec2,
    },
}

pub struct Aabb {
    pub min: glam::Vec2,
    pub max: glam::Vec2,
}

impl Aabb {
    pub fn zero() -> Self {
        Aabb {
            min: glam::Vec2::new(0.0, 0.0),
            max: glam::Vec2::new(0.0, 0.0),
        }
    }

    pub fn union(&self, other: &Aabb) -> Aabb {
        Aabb {
            min: glam::Vec2::new(
                self.min.x().min(other.min.x()),
                self.min.y().min(other.min.y()),
            ),
            max: glam::Vec2::new(
                self.max.x().max(other.max.x()),
                self.max.y().max(other.max.y()),
            ),
        }
    }

    pub fn from_curves(curves: &[Curve]) -> Self {
        curves
            .iter()
            .fold(Aabb::zero(), |aabb, curve| aabb.union(&curve.aabb()))
    }

    pub fn from_segments(segments: &[Segment]) -> Self {
        segments.iter().fold(Aabb::zero(), |aabb, segment| {
            aabb.union(&Aabb::from_curves(&segment))
        })
    }
}

impl Curve {
    pub fn aabb(&self) -> Aabb {
        match *self {
            Curve::Line { p0, p1 } => Aabb {
                min: glam::Vec2::new(p0.x().min(p1.x()), p0.y().min(p1.y())),
                max: glam::Vec2::new(p0.x().max(p1.x()), p0.y().max(p1.y())),
            },
            Curve::Quad { p0, p1, p2 } => {
                // bad approx
                Aabb {
                    min: glam::Vec2::new(
                        p0.x().min(p1.x()).min(p2.x()),
                        p0.y().min(p1.y()).min(p2.y()),
                    ),
                    max: glam::Vec2::new(
                        p0.x().max(p1.x()).max(p2.x()),
                        p0.y().max(p1.y()).max(p2.y()),
                    ),
                }
            }
        }
    }
}

pub fn line_eval(p0: f32, p1: f32, t: f32) -> f32 {
    (1.0 - t) * p0 + t * p1
}
