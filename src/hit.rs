use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

#[derive(Clone, Debug, PartialEq)]
pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };

        (front_face, normal)
    }

    pub fn point(&self) -> Point3 {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn t(&self) -> f64 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();        
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);

        let (front_face, normal) = HitRecord::face_normal(ray,
            (point - self.center) / self.radius);

        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
        })
    }
}