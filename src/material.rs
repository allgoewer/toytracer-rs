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

#[derive(Clone, Debug, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let scatter_direction = hr.normal() + Vec3::random_unit_vector();

        // catch degenerate scatter direction
        let scatter_direction = if scatter_direction.near_zero() {
            hr.normal()
        } else {
            scatter_direction
        };

        Some(Scatter {
            attenuation: self.albedo,
            scattered: Ray::new(hr.point(), scatter_direction),
        })
    }
}