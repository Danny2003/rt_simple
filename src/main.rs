///
/// The progress bar, information on the terminal, and the multithread part is borrowed from the following person:
/// [@PaperL](https://github.com/PaperL/), an ACM Class TA
/// He is the author of the [PPCA-Raytracer-2022](https://github.com/ACMClassCourse-2021/PPCA-Raytracer-2022) project.
///
extern crate rand;
pub mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod cornell_box;
mod hit;
mod material;
mod perlin;
mod scene;
mod texture;
pub use camera::Camera;
pub use hit::*;
use material::*;
mod rt_weekend;
pub use rt_weekend::*;
mod sphere;
use color::write_color;
mod ray;
mod vec3;
use crate::bvh::BVHNode;
use crate::scene::*;
pub use ray::Ray;
use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Instant;
use std::{f64::INFINITY, fs::File, process::exit};
pub use vec3::Vec3;

use console::style;
use image::{ImageBuffer, RgbImage};
#[allow(unused_imports)]
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
/// ray_color() function decides the color of a ray.
fn ray_color(r: Ray, background: &Vec3, world: &Arc<BVHNode>, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }
    let mut hit_record = HitRecord::new(Arc::new(Lambertian::new(Vec3::zero())));
    // Fixing Shadow Acne by setting t_min 0.001.
    // If the ray hits nothing, return the background color.
    if !world.hit(&r, 0.001, INFINITY, &mut hit_record) {
        return *background;
    }
    let mut scattered = Ray::zero();
    let mut attenuation = Vec3::zero();
    let emitted = hit_record
        .material
        .emitted(hit_record.u, hit_record.v, &hit_record.p);
    if !hit_record
        .material
        .scatter(&r, &hit_record, &mut attenuation, &mut scattered)
    {
        return emitted;
    }
    emitted
        + Vec3::elemul(
            attenuation,
            ray_color(scattered, background, world, depth - 1),
        )
    // return Vec3::zero();

    // let unit_direction = Vec3::unit(r.direction());
    // let t = 0.5 * (unit_direction.y() + 1.0);
    // Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

//---------------------------------------------------------------------------------

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1

    println!(
        "{} 💿 {}",
        style("[1/5]").bold().dim(),
        style("Initializing...").green()
    );
    let begin_time = Instant::now();

    const THREAD_NUMBER: usize = 16;

    const AUTHOR: &str = "Youwei Zhong";
    const PATH: &str = "output/Cornell_box_with_two_blocks.jpg";

    //---------------------------------------------------------------------------------

    // Image

    let mut aspect_ratio: f64 = 16.0 / 9.0;
    let mut image_width: usize = 400;
    let mut samples_per_pixel: usize = 100;
    /// Reflection max depth
    const MAX_DEPTH: i32 = 50;
    /// JPG_QUALITY
    /// From 0 to 100
    const QUALITY: u8 = 100;

    // Scene

    let hit_list: Arc<HitList>;
    // let look_from = Vec3::new(3., 3., 2.);
    let look_from: Vec3;
    // let look_at = Vec3::new(0., 0., -1.);
    let look_at: Vec3;
    let vfov: f64;
    // * `aperture` - aperture's radius of the camera
    let mut aperture = 0.;
    let background;
    match 6 {
        1 => {
            hit_list = Arc::new(random_scene());
            background = Vec3::new(0.7, 0.8, 1.);
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            hit_list = Arc::new(two_spheres());
            background = Vec3::new(0.7, 0.8, 1.);
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
        }
        3 => {
            hit_list = Arc::new(two_perlin_spheres());
            background = Vec3::new(0.7, 0.8, 1.);
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
        }
        4 => {
            hit_list = Arc::new(earth());
            background = Vec3::new(0.7, 0.8, 1.);
            look_from = Vec3::new(13., 2., 3.);
            look_at = Vec3::new(0., 0., 0.);
            vfov = 20.0;
        }
        5 => {
            hit_list = Arc::new(simple_light());
            samples_per_pixel = 400;
            background = Vec3::zero();
            look_from = Vec3::new(26., 3., 6.);
            look_at = Vec3::new(0., 2., 0.);
            vfov = 20.0;
        }
        6 => {
            hit_list = Arc::new(cornell_box());
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Vec3::zero();
            look_from = Vec3::new(278., 278., -800.);
            look_at = Vec3::new(278., 278., 0.);
            vfov = 40.0;
        }
        _ => {
            hit_list = Arc::new(HitList::new());
            background = Vec3::zero();
            look_from = Vec3::zero();
            look_at = Vec3::zero();
            vfov = 40.0;
        }
    }
    let world = Arc::new(BVHNode::new(
        &mut hit_list.list.clone(),
        0,
        hit_list.list.len(),
        0.,
        1.,
    ));

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

    let vup = Vec3::new(0., 1., 0.);
    // let dist_to_focus = (look_from - look_at).length();
    let dist_to_focus = 10.;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    println!(
        "Image size: {}\nJPEG quality: {}\nSamples per pixel: {}\nReflection max depth: {}",
        style(image_width.to_string() + &'x'.to_string() + &image_height.to_string()).yellow(),
        style(QUALITY.to_string()).yellow(),
        style(samples_per_pixel.to_string()).yellow(),
        style(MAX_DEPTH.to_string()).yellow()
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(
        image_width.try_into().unwrap(),
        image_height.try_into().unwrap(),
    );
    let camera = Arc::new(Camera::new(
        look_from,
        look_at,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    ));

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

    //========================================================

    println!(
        "{} 🚀 {} {} {}",
        style("[2/5]").bold().dim(),
        style("Rendering with").green(),
        style(THREAD_NUMBER.to_string()).yellow(),
        style("Threads...").green(),
    );

    let section_line_num: usize = image_height as usize / THREAD_NUMBER;

    let mut output_pixel_color = Vec::<Vec3>::new();
    let mut thread_pool = VecDeque::<_>::new();
    // Manages multiple progress bars from different threads
    // let multiprogress = Arc::new(MultiProgress::new());
    // multiprogress.set_move_cursor(true); // turn on this to reduce flickering

    for thread_id in 0..THREAD_NUMBER {
        let line_beg = section_line_num * thread_id;
        let line_end = if line_beg + section_line_num > image_height
            || (thread_id == THREAD_NUMBER - 1 && line_beg + section_line_num < image_height)
        {
            image_height
        } else {
            line_beg + section_line_num
        };
        // let mp = multiprogress.clone();
        // // Progress bar UI powered by library `indicatif`
        // // Get environment variable CI, which is true for GitHub Action
        // let progress_bar = if option_env!("CI").unwrap_or_default() == "true" {
        //     ProgressBar::hidden()
        // } else {
        //     mp.add(ProgressBar::new((line_end - line_beg) as u64))
        // };
        // progress_bar.set_style(ProgressStyle::default_bar()
        // .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        // .progress_chars("#>-"));

        let (tx, rx) = mpsc::channel();
        let camera_clone = camera.clone();
        let world_clone = world.clone();
        thread_pool.push_back((
            thread::spawn(move || {
                // let mut progress = 0;
                // progress_bar.set_position(0);

                let channel_send = tx.clone();

                let mut section_pixel_color = Vec::<Vec3>::new();

                for j in line_beg..line_end {
                    for i in 0..image_width {
                        let mut pixel_color = Vec3::zero();
                        // take samples_per_pixel samples and average them
                        for _s in 0..samples_per_pixel {
                            let u = (i as f64 + random_double()) / (image_width as f64);
                            let v = (j as f64 + random_double()) / (image_height as f64);
                            let r = camera_clone.get_ray(u, v);
                            pixel_color += ray_color(r, &background, &world_clone, MAX_DEPTH);
                        }
                        section_pixel_color.push(pixel_color);
                    }
                    // progress += 1;
                    // progress_bar.set_position(progress);
                }
                channel_send.send(section_pixel_color).unwrap();
                // progress_bar.finish_with_message("Finished.");
            }),
            rx,
        ));
    }
    // 等待所有线程结束
    // multiprogress.join().unwrap();

    //========================================================

    println!(
        "{} 🚛 {}",
        style("[3/5]").bold().dim(),
        style("Collecting Threads Results...").green(),
    );

    let mut thread_finish_successfully = true;
    let collecting_progress_bar = ProgressBar::new(THREAD_NUMBER as u64);
    // join 和 recv 均会阻塞主线程
    for thread_id in 0..THREAD_NUMBER {
        let thread = thread_pool.pop_front().unwrap();
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                collecting_progress_bar.inc(1);
            }
            Err(_) => {
                thread_finish_successfully = false;
                println!(
                    "      ⚠️ {}{}{}",
                    style("Joining the ").red(),
                    style(thread_id.to_string()).yellow(),
                    style("th thread failed!").red(),
                );
            }
        }
    }
    if !thread_finish_successfully {
        exit_with_error("Get run-time error!");
    }
    collecting_progress_bar.finish_and_clear();

    //========================================================

    println!(
        "{} 🏭 {}",
        style("[4/5]").bold().dim(),
        style("Generating Image...").green()
    );

    let mut pixel_id = 0;
    for j in 0..image_height {
        for i in 0..image_width {
            write_color(
                // + halo[y as usize][x as usize];
                output_pixel_color[pixel_id],
                samples_per_pixel,
                &mut img,
                i,
                image_height - j - 1,
            );
            pixel_id += 1;
        }
    }

    //========================================================

    println!(
        "{} 🥽 {}",
        style("[5/5]").bold().dim(),
        style("Outputting Image...").green()
    );

    // Output image to file
    println!("Output image as \"{}\"", style(PATH).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(PATH).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(QUALITY)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    //========================================================

    println!(
        "\n      🎉 {}\n      🕒 Elapsed Time: {}\n      🧑‍💻 Author: {}",
        style("All Work Done.").bold().green(),
        style(HumanDuration(begin_time.elapsed())).yellow(),
        style(AUTHOR).bold().blue(),
    );
    println!("\n");
    exit(0);
}

fn exit_with_error<T>(info: T)
where
    T: Display,
{
    println!(
        "\n\n      {}{}\n\n",
        style("❌ Error: ").bold().red().on_yellow(),
        style(info).black().on_yellow()
    );
    exit(1);
}
