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
use sphere::*;
mod ray;
mod vec3;
pub use ray::Ray;
use std::sync::Arc;
use std::{f64::INFINITY, fs::File, process::exit};
pub use vec3::Vec3;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};

fn random_scene() -> HitList {
    let mut world = HitList::new();

    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            // choose material
            let choose_mat = random_double();
            let center = Vec3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            // 1.2^2 -0.8^2 < 0.9^2 to prevent being too close to the right sphere
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse: 80%
                    let albedo = Vec3::elemul(Vec3::random(), Vec3::random());
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal: 15%
                    let albedo = Vec3::random_in_range(0.5, 1.);
                    let fuzz = random_double_in_range(0., 0.5);
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass: 5%
                    Arc::new(Dielectric::new(1.5))
                };
                if choose_mat < 0.8 {
                    let center2 = center + Vec3::new(0., random_double_in_range(0., 0.5), 0.);
                    world.add(Box::new(MovingSphere::new(
                        center,
                        center2,
                        0.,
                        1.,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    // centre
    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1., material1)));
    // left
    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., material2)));
    // right
    let material3 = Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.));
    world.add(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., material3)));

    world
}
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
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
    let quality = 100; // From 0 to 100

    let author = "Youwei Zhong";
    let path = "output/Bouncing_spheres.jpg";

    // Image

    let aspect_ratio = 16.0 / 9.0;
    // let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    // let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio).floor() as u32;
    let samples_per_pixel = 100;
    // let samples_per_pixel = 500;
    let max_depth = 50;

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(image_width.to_string() + &'x'.to_string() + &image_height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(image_height as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));
    progress.set_message("Rendering...");

    // World

    let world = random_scene();

    // let r = (PI / 4.0).cos();

    // Image 20: Spheres with depth-of-field

    // let mut world = hit::HitList::new();
    // let material_ground = Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    // let material_center = Arc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    // let material_left = Arc::new(Dielectric::new(1.5));
    // let material_right = Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    // world.add(Box::new(sphere::Sphere::new(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left.clone(),
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     material_left,
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     Vec3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));

    // Camera

    // let look_from = Vec3::new(3., 3., 2.);
    let look_from = Vec3::new(13., 2., 3.);
    // let look_at = Vec3::new(0., 0., -1.);
    let look_at = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    // let dist_to_focus = (look_from - look_at).length();
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    );

    // Render

    // ppm format
    // file.write_all(b"P3\n").expect("wrong write");
    // file.write_all(b" ").expect("wrong write");
    // file.write_all(image_width.to_string().as_bytes())
    //     .expect("wrong write");
    // file.write_all(b" ").expect("wrong write");
    // file.write_all(image_height.to_string().as_bytes())
    //     .expect("wrong write");
    // file.write_all(b"\n255\n").expect("wrong write");

    // let bar = ProgressBar::new((image_height + 1) as u64);
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
            write_color(
                pixel_color,
                samples_per_pixel,
                &mut img,
                i,
                image_height - j - 1,
            );
        }
        progress.inc(1);
    }
    progress.finish();

    // Output image to file
    println!("Output image as \"{}\"", style(path).yellow());
    println!("Done!\nAuthor: {}", author);
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
