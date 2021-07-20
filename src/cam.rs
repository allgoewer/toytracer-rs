use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    viewport_height: f64,
    focal_length: f64,
    origin: Point3,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            viewport_height: 2.0,
            focal_length: 1.0,
            origin: Point3::new(0.0, 0.0, 0.0),
        }
    }
}

impl CameraBuilder {
    pub fn aspect_ratio(mut self, aspect_ratio: f64) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn viewport_height(mut self, viewport_height: f64) -> Self {
        self.viewport_height = viewport_height;
        self
    }

    pub fn focal_length(mut self, focal_length: f64) -> Self {
        self.focal_length = focal_length;
        self
    }

    pub fn origin(mut self, origin: Point3) -> Self {
        self.origin = origin;
        self
    }

    pub fn build(self) -> Camera {
        let viewport_width = self.aspect_ratio * self.viewport_height;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, self.viewport_height, 0.0);

        Camera {
            origin: self.origin,
            horizontal,
            vertical,
            lower_left_corner: self.origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, self.focal_length),
        }
    }
}
