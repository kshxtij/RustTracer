mod hittable;
mod ray;
mod sphere;
mod camera;
mod util;

use image::{imageops, ImageBuffer, RgbImage};
use nalgebra::Vector3;
use std::env;
use std::time::Instant;
use rand::Rng;

use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::camera::Camera;
use crate::util::format_string;

fn color(ray: &Ray, world: &HittableList) -> Vector3<f32> {
    if let Some(hit) = world.hit(ray, 0.0, f32::MAX) {
        0.5 * hit.normal.add_scalar(1.0)
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
    // let ny = args[2].parse::<u32>().unwrap();
    let aspect_ratio = 16.0/9.0;
    let ny = nx as f32 / aspect_ratio;
    let ns = args[2].parse::<u32>().unwrap();
    let cam = Camera::new();
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    let mut img: RgbImage = ImageBuffer::new(nx as u32, ny as u32);
    let start = Instant::now();
    for j in 0..(ny as u32) {
        for i in 0..(nx as u32) {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx;
                let v = (j as f32 + rng.gen::<f32>()) / ny;
                let ray = cam.get_ray(u, v);
                col += color(&ray, &world);
            }
            col /= ns as f32;
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
