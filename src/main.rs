mod color;
use color::write_color;
mod ray;
mod vec3;
pub use ray::Ray;
use std::{fs::File, io::Write};
pub use vec3::Vec3;
fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let oc = r.origin() - center;
    let a = r.direction().squared_length();
    let half_b = oc * r.direction();
    let c = oc.squared_length() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
fn ray_color(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t != -1.0 {
        let n = Vec3::unit(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5
    } else {
        let unit_direction = Vec3::unit(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}
fn main() {
    let file_name = "output/A_sphere_colored_according_to_its_normal_vectors.ppm";
    let mut file = File::create(file_name).unwrap();

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).floor() as i32;

    // Camera

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::zero();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_color = ray_color(r);
            write_color(pixel_color, &mut file);
        }
    }
}
