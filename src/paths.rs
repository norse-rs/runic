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

pub struct PathBuilder {
    curves: Vec<Curve>,
    first: glam::Vec2,
    last: glam::Vec2,
}

impl PathBuilder {
    pub fn new() -> Self {
        PathBuilder {
            curves: Vec::new(),
            first: glam::vec2(0.0, 0.0),
            last: glam::vec2(0.0, 0.0),
        }
    }

    pub fn move_to(mut self, p: glam::Vec2) -> Self {
        self.first = p;
        self.last = p;
        self
    }

    pub fn line_to(mut self, p: glam::Vec2) -> Self {
        self.curves.push(Curve::Line {
            p0: self.last,
            p1: p,
        });
        self.last = p;
        self
    }

    pub fn close(mut self) -> Self {
        self.curves.push(Curve::Line {
            p0: self.last,
            p1: self.first,
        });
        self.last = self.first;
        self
    }

    pub fn finish(self) -> Vec<Curve> {
        self.curves
    }
}
