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

    pub fn eval(&self, t: f32) -> glam::Vec2 {
        match *self {
            Curve::Line { p0, p1 } => (1.0 - t) * p0 + t * p1,
            Curve::Quad { p0, p1, p2 } => {
                (1.0 - t) * (1.0 - t) * p0 + 2.0 * t * (1.0 - t) * p1 + t * t * p2
            }
        }
    }

    pub fn monotonize(&self) -> Vec<Curve> {
        match *self {
            Curve::Line { .. } => vec![*self],
            Curve::Quad { p0, p1, p2 } => {
                let min = glam::vec2(p0.x().min(p2.x()), p0.y().min(p2.y()));
                let max = glam::vec2(p0.x().max(p2.x()), p0.y().max(p2.y()));

                let tx = if p1.x() < min.x() || max.x() < p1.x() {
                    Some((p0.x() - p1.x()) / (p0.x() - 2.0 * p1.x() + p2.x()))
                } else {
                    None
                };

                let ty = if p1.y() < min.y() || max.y() < p1.y() {
                    Some((p0.y() - p1.y()) / (p0.y() - 2.0 * p1.y() + p2.y()))
                } else {
                    None
                };

                match (tx, ty) {
                    (Some(tx), Some(ty)) => {
                        let t = tx.min(ty);
                        let p = self.eval(t);
                        let p10 = Curve::Line { p0, p1 }.eval(t);
                        let p11 = Curve::Line { p0: p1, p1: p2 }.eval(t);
                        let mut curves = vec![Curve::Quad { p0, p1: p10, p2: p }];
                        curves.extend(Curve::Quad { p0: p, p1: p11, p2 }.monotonize());
                        curves
                    }
                    (Some(t), None) | (None, Some(t)) => {
                        let p = self.eval(t);
                        let p10 = Curve::Line { p0, p1 }.eval(t);
                        let p11 = Curve::Line { p0: p1, p1: p2 }.eval(t);
                        vec![
                            Curve::Quad { p0, p1: p10, p2: p },
                            Curve::Quad { p0: p, p1: p11, p2 },
                        ]
                    }
                    (None, None) => vec![*self],
                }
            }
        }
    }

    pub fn monotize_path(curves: &[Curve]) -> Vec<Curve> {
        curves
            .iter()
            .map(|curve| curve.monotonize())
            .flatten()
            .collect()
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

    pub fn quad_to(mut self, p1: glam::Vec2, p2: glam::Vec2) -> Self {
        self.curves.push(Curve::Quad {
            p0: self.last,
            p1,
            p2,
        });
        self.last = p2;
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

    pub fn monotonize(mut self) -> Self {
        self.curves = Curve::monotize_path(&self.curves);
        self
    }

    pub fn finish(self) -> Vec<Curve> {
        self.curves
    }
}
