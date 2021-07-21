use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug)]
pub struct HitRecord<'mat> {
    point: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    mat: &'mat dyn Material,
}

impl<'mat> HitRecord<'mat> {
    pub fn face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

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

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn mat(&self) -> &dyn Material {
        &*self.mat
    }
}

impl<H: Hittable> Hittable for &[H] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut latest_hit = None;
        let mut closest_so_far = t_max;

        for hittable in *self {
            if let Some(hr) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.t();
                latest_hit = Some(hr);
            }
        }

        latest_hit
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone, Debug)]
pub struct Sphere<'mat> {
    center: Point3,
    radius: f64,
    mat: &'mat dyn Material,
}

impl<'mat> Sphere<'mat> {
    pub fn new(center: Point3, radius: f64, material: &'mat dyn Material) -> Self {
        Self {
            center,
            radius,
            mat: material,
        }
    }
}

impl Hittable for Sphere<'_> {
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

        let (front_face, normal) = HitRecord::face_normal(ray, (point - self.center) / self.radius);

        Some(HitRecord {
            point,
            normal,
            t,
            front_face,
            mat: self.mat.clone(),
        })
    }
}
