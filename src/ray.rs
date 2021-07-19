use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Create a new Ray
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Returns the origin of a Ray
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// Returns the direction of a Ray
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Returns the point a Ray reaches at "time" t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}
