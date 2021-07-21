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

#[derive(Clone, Debug, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let reflected = ray.direction().unit().reflect(hr.normal());
        let scattered = Ray::new(hr.point(), reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.direction().dot(hr.normal()) <= 0.0 {
            None
        } else {
            Some(Scatter {
                attenuation: self.albedo,
                scattered,
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            index_of_refraction,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if hr.front_face() {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction().unit();
        let refracted = unit_direction.refract(hr.normal(), refraction_ratio);

        Some(Scatter {
            attenuation: Color::new(1.0, 1.0, 1.0),
            scattered: Ray::new(hr.point(), refracted),
        })
    }
}