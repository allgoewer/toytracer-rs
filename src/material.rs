use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rand::prelude::*;

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

#[derive(Clone, Debug, PartialEq)]
pub enum Material {
    Lambertian {
        albedo: Color,
    },
    Metal {
        albedo: Color,
        fuzz: f64,
    },
    Dielectric {
        index_of_refraction: f64,
    }
}

impl Material {
    pub fn new_lambertian(albedo: Color) -> Self {
        Self::Lambertian {
            albedo,
        }
    }

    pub fn new_metal(albedo: Color, fuzz: f64) -> Self {
        Self::Metal {
            albedo,
            fuzz,
        }
    }

    pub fn new_dielectric(index_of_refraction: f64) -> Self {
        Self::Dielectric {
            index_of_refraction,
        }
    }

    pub fn scatter(&self, ray: &Ray, hr: &HitRecord) -> Option<Scatter> {
        match self {
            Self::Lambertian { albedo } => {
                let scatter_direction = hr.normal() + Vec3::random_unit_vector();

                // catch degenerate scatter direction
                let scatter_direction = if scatter_direction.near_zero() {
                    hr.normal()
                } else {
                    scatter_direction
                };

                Some(Scatter {
                    attenuation: *albedo,
                    scattered: Ray::new(hr.point(), scatter_direction),
                })
            }
            Self::Metal { albedo, fuzz } => {
                let reflection = ray.direction().unit().reflect(hr.normal());
                let scattered = Ray::new(hr.point(), reflection + *fuzz * Vec3::random_in_unit_sphere());

                if scattered.direction().dot(hr.normal()) <= 0.0 {
                    None
                } else {
                    Some(Scatter {
                        attenuation: *albedo,
                        scattered,
                    })
                }
            }
            Self::Dielectric { index_of_refraction } => {
                let refraction_ratio = if hr.front_face() {
                    1.0 / *index_of_refraction
                } else {
                    *index_of_refraction
                };

                let unit_direction = ray.direction().unit();
                let cos_theta = (-unit_direction).dot(hr.normal()).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let direction = if refraction_ratio * sin_theta > 1.0 || self.reflectance(cos_theta, refraction_ratio) > thread_rng().gen_range(0.0..1.0)
                {
                    unit_direction.reflect(hr.normal())
                } else {
                    unit_direction.refract(hr.normal(), refraction_ratio)
                };

                Some(Scatter {
                    attenuation: Color::new(1.0, 1.0, 1.0),
                    scattered: Ray::new(hr.point(), direction),
                })
            }
        }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}