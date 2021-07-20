#![forbid(unsafe_code)]
#![allow(dead_code)]

mod ray;
mod vec3;

use std::io;
use ray::Ray;
use vec3::{Color, Point3, Vec3};

pub fn to_ppm<W: io::Write>(
    w: &mut W,
    values: impl Iterator<Item = Color>,
    width: usize,
    height: usize,
) -> io::Result<()> {
    writeln!(w, "P3")?;
    writeln!(w, "# Raytraced image generated by Maiks raytracer")?;
    writeln!(w, "{} {}\n255\n", width, height)?;

    for color in values {
        let (r, g, b) = color.xyz();

        let r = (r * 255.999) as u8;
        let g = (g * 255.999) as u8;
        let b = (b * 255.999) as u8;

        writeln!(w, "{} {} {}", r, g, b)?;
    }

    Ok(())
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);

    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
        0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0)
    } else {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn main() -> std::io::Result<()> {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    eprintln!("camera:     {:?}", origin);
    eprintln!("horizontal: {:?}", horizontal);
    eprintln!("vertical:   {:?}", vertical);
    eprintln!("ll_corner:  {:?}", lower_left_corner);

    let img: Vec<_> = (0..image_height)
        .rev()
        .map(move |j| {
            eprintln!("{:4} / {:4} lines remaining", j + 1, image_height);
            (0..image_width).map(move |i| {
                let u = i as f64 / (image_width - 1) as f64;
                let v = j as f64 / (image_height - 1) as f64;

                let ray = Ray::new(
                    origin,
                    lower_left_corner + horizontal * u + vertical * v - origin,
                );

                ray_color(&ray)
            })
        })
        .flatten()
        .collect();

    let stdout = std::io::stdout();
    to_ppm(
        &mut stdout.lock(),
        img.into_iter(),
        image_width,
        image_height,
    )?;

    Ok(())
}
