mod hittable;
mod ray;
mod sphere;
mod util;

use image::{imageops, ImageBuffer, RgbImage};
use nalgebra::Vector3;
use std::env;
use std::time::Instant;

use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::sphere::Sphere;
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
    let args: Vec<String> = env::args().collect();
    let nx = args[1].parse::<u32>().unwrap();
    let ny = args[2].parse::<u32>().unwrap();
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);
    let world = HittableList::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
    ]);
    let mut img: RgbImage = ImageBuffer::new(nx, ny);
    let start = Instant::now();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let ray = Ray::new(origin, lower_left_corner + (i as f32 / nx as f32) * horizontal + (j as f32 / ny as f32) * vertical,);
            let col = color(&ray, &world);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            img.put_pixel(i, j, image::Rgb([ir as u8, ig as u8, ib as u8]))
        }
    }
    let duration = start.elapsed();
    imageops::flip_vertical(&img)
        .save(format!("images/{}x{}.{:}.png", nx, ny, format_string(duration)))
        .unwrap();
}
