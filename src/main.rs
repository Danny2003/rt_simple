extern crate rand;
mod camera;
mod color;
mod hit;
pub use camera::Camera;
pub use hit::*;
mod rt_weekend;
pub use rt_weekend::*;
mod sphere;
use color::write_color;
mod ray;
mod vec3;
pub use ray::Ray;
use std::{f64::INFINITY, fs::File, io::Write};
pub use vec3::Vec3;
// ray_color() function decides the color of a ray.
fn ray_color(r: Ray, world: &hit::HitList) -> Vec3 {
    let mut hit_record = hit::HitRecord::zero();
    if world.hit(r, 0.0, INFINITY, &mut hit_record) {
        return (hit_record.normal + Vec3::ones()) * 0.5;
    }
    let unit_direction = Vec3::unit(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
fn main() {
    let file_name = "output/normals-colored_sphere_with_ground_with_anti-aliasing.ppm";
    let mut file = File::create(file_name).unwrap();

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
    let samples_per_pixel = 100;
    // World

    let mut world = hit::HitList::new();
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera

    let camera = Camera::new();

    // Render

    file.write_all(b"P3\n").expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all(image_width.to_string().as_bytes())
        .expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all(image_height.to_string().as_bytes())
        .expect("wrong write");
    file.write_all(b"\n255\n").expect("wrong write");

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::zero();
            // take samples_per_pixel samples and average them
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64);
                let v = (j as f64 + random_double()) / (image_height as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world);
            }
            write_color(pixel_color, &mut file, samples_per_pixel);
        }
    }
}
