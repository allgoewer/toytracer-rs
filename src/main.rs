#![forbid(unsafe_code)]
#![allow(dead_code)]

mod cam;
mod hit;
mod ray;
mod vec3;

use cam::CameraBuilder;
use hit::{Hittable, Sphere};
use rand::prelude::*;
use ray::Ray;
use std::io;
use vec3::{Color, Point3, Vec3};

pub fn to_ppm<W: io::Write>(
    w: &mut W,
    values: impl Iterator<Item = Color>,
    width: usize,
    height: usize,
    samples_per_pixel: u32,
) -> io::Result<()> {
    writeln!(w, "P3")?;
    writeln!(w, "# Raytraced image generated by Maiks raytracer")?;
    writeln!(w, "{} {}\n255\n", width, height)?;

    let scale = 1.0 / samples_per_pixel as f64;

    for color in values {
        let (r, g, b) = color.xyz();

        // gamma-correct for gamma = 2.0, scale and clamp the rgb values
        let r = (r * scale).sqrt().clamp(0.0, 0.999);
        let g = (g * scale).sqrt().clamp(0.0, 0.999);
        let b = (b * scale).sqrt().clamp(0.0, 0.999);

        // convert rgb to u8
        let r = (r * 255.999) as u8;
        let g = (g * 255.999) as u8;
        let b = (b * 255.999) as u8;

        writeln!(w, "{} {} {}", r, g, b)?;
    }

    Ok(())
}

fn ray_color<H: Hittable>(ray: &Ray, world: H, depth: u32) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hr = world.hit(&ray, 0.001, f64::INFINITY);

    if let Some(hr) = hr {
        let point = hr.point();
        let target = point + hr.normal() + Vec3::random_unit_vector();

        0.5 * ray_color(&Ray::new(point, target - point), world, depth - 1)
    } else {
        let unit_direction = ray.direction().unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() -> std::io::Result<()> {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // world
    let mut world = Vec::new();
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // camera
    let camera = &CameraBuilder::default().build();

    eprintln!("camera:     {:?}", camera);

    let world = &world;
    let img: Vec<_> = (0..image_height)
        .rev()
        .map(move |j| {
            eprintln!("{:4} / {:4} lines remaining", j + 1, image_height);
            (0..image_width).map(move |i| {
                let mut color = Color::new(0.0, 0.0, 0.0);
                let mut rng = thread_rng();

                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen_range(0.0..1.0)) / (image_width - 1) as f64;
                    let v = (j as f64 + rng.gen_range(0.0..1.0)) / (image_height - 1) as f64;

                    let ray = camera.get_ray(u, v);
                    color += ray_color(&ray, &world[..], max_depth);
                }

                color
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
        samples_per_pixel,
    )?;

    Ok(())
}
