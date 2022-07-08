extern crate rand;
mod camera;
mod color;
mod hit;
mod material;
pub use camera::Camera;
pub use hit::*;
use material::*;
mod rt_weekend;
pub use rt_weekend::*;
mod sphere;
use color::write_color;
mod ray;
mod vec3;
use indicatif::ProgressBar;
pub use ray::Ray;
use std::sync::Arc;
use std::{f64::INFINITY, fs::File, io::Write};
pub use vec3::Vec3;
/// ray_color() function decides the color of a ray.
fn ray_color(r: Ray, world: &hit::HitList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    let mut hit_record = HitRecord::new(Arc::new(Lambertian::new(Vec3::zero())));
    // Fixing Shadow Acne by setting t_min 0.001.
    if world.hit(r, 0.001, INFINITY, &mut hit_record) {
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();
        if hit_record
            .material
            .scatter(&r, &hit_record, &mut attenuation, &mut scattered)
        {
            return Vec3::elemul(attenuation, ray_color(scattered, world, depth - 1));
        }
        return Vec3::zero();
    }
    let unit_direction = Vec3::unit(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
fn main() {
    let author = "Youwei Zhong";
    let file_name = "output/Zooming_in.ppm";
    let mut file = File::create(file_name).unwrap();

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    // let r = (PI / 4.0).cos();
    let mut world = hit::HitList::new();
    let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Box::new(sphere::Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera

    let camera = Camera::new(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
    );

    // Render

    file.write_all(b"P3\n").expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all(image_width.to_string().as_bytes())
        .expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all(image_height.to_string().as_bytes())
        .expect("wrong write");
    file.write_all(b"\n255\n").expect("wrong write");

    let bar = ProgressBar::new((image_height + 1) as u64);
    println!("CI: false, multitask: false\nRendering...",);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Vec3::zero();
            // take samples_per_pixel samples and average them
            for _s in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width as f64);
                let v = (j as f64 + random_double()) / (image_height as f64);
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, &mut file, samples_per_pixel);
        }
        bar.inc(1);
    }
    bar.finish();
    println!("Done!\nAuthor: {}", author);
}
