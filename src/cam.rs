use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    lens_radius: f64,
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lower_left_corner: Point3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CameraBuilder {
    vfov: f64,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            vfov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            aperture: 1.0,
            focus_dist: 1.0,
            look_from: Point3::new(0.0, 0.0, 0.0),
            look_at: Point3::new(0.0, 0.0, 1.0),
            vup: Point3::new(0.0, 1.0, 0.0),
        }
    }
}

impl CameraBuilder {
    pub fn vertical_fov(&mut self, vfov: f64) -> &mut Self {
        self.vfov = vfov;
        self
    }

    pub fn aspect_ratio(&mut self, aspect_ratio: f64) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn aperture(&mut self, aperture: f64) -> &mut Self {
        self.aperture = aperture;
        self
    }

    pub fn focus_dist(&mut self, focus_dist: f64) -> &mut Self {
        self.focus_dist = focus_dist;
        self
    }

    pub fn look_from(&mut self, look_from: Point3) -> &mut Self {
        self.look_from = look_from;
        self
    }

    pub fn look_at(&mut self, look_at: Point3) -> &mut Self {
        self.look_at = look_at;
        self
    }

    pub fn view_up(&mut self, view_up: Vec3) -> &mut Self {
        self.vup = view_up;
        self
    }

    pub fn build(&self) -> Camera {
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = (self.look_from - self.look_at).unit();
        let u = self.vup.cross(w).unit();
        let v = w.cross(u);

        let horizontal = self.focus_dist * viewport_width * u;
        let vertical = self.focus_dist * viewport_height * v;

        Camera {
            lens_radius: self.aperture / 2.0,
            origin: self.look_from,
            horizontal,
            vertical,
            u,
            v,
            w,
            lower_left_corner: self.look_from
                - horizontal / 2.0
                - vertical / 2.0
                - self.focus_dist * w,
        }
    }
}
