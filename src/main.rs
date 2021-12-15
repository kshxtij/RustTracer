mod hittable;
mod ray;
mod material;
mod sphere;
mod camera;
mod util;

use std::f32;
use image::{imageops, ImageBuffer, RgbImage};
use std::time::Instant;
use std::env;
use nalgebra::Vector3;
use rand::Rng;
use crate::ray::Ray;
use crate::material::{Lambertian, Metal};
use crate::hittable::{Hittable, HittableList};
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::util::format_string;

fn color(ray: &Ray, world: &HittableList, depth: i32) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit.material.scatter(&ray, &hit) {
                return attenuation.zip_map(&color(&scattered, &world, depth+1), |l, r| l * r);
            }
        }
        Vector3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction[1] + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let args: Vec<String> = env::args().collect();
    let nx = args[1].parse::<f32>().unwrap();
    let aspect_ratio = 16.0/9.0;
    let ny = nx as f32 / aspect_ratio;
    let ns = args[2].parse::<u32>().unwrap();
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Vector3::new(0.8, 0.3, 0.3)))),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Vector3::new(0.8, 0.8, 0.0)))),
        Box::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Metal::new(Vector3::new(0.8, 0.6, 0.2), 1.0))),
        Box::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Metal::new(Vector3::new(0.8, 0.8, 0.8), 0.3)))
    ]);
    let cam = Camera::new();
    let mut img: RgbImage = ImageBuffer::new(nx as u32, ny as u32);
    let start = Instant::now();
    for j in (0..ny as u32).rev() {
        for i in 0..nx as u32 {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world, 0);
            }
            col /= ns as f32;
            for c in col.iter_mut() { *c = c.sqrt(); }
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            img.put_pixel(i, j, image::Rgb([ir as u8, ig as u8, ib as u8]))
        }
    }
    let duration = start.elapsed();
    imageops::flip_vertical(&img)
    .save(format!("images/{}x{}:{:}.png", nx, ny, format_string(duration)))
    .unwrap();
}