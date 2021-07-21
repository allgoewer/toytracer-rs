use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Scatter {
    attenuation: Color,
    scattered: Ray,
}

impl Scatter {
    pub fn attenuation(&self) -> Color {
        self.attenuation
    }

    pub fn scattered(&self) -> &Ray {
        &self.scattered
    }
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter>;
}
