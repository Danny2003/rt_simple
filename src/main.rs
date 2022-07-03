mod color;
use color::write_color;
mod vec3;
use std::{fs::File, io::Write};
pub use vec3::Vec3;
fn main() {
    let file_name = "output/basic_ppm.ppm";
    let mut file = File::create(file_name).unwrap();
    // blue2white.ppm
    let image_width: i32 = 256;
    let image_height: i32 = 256;
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
            let pixel_color = Vec3::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );
            write_color(pixel_color, &mut file);
        }
    }
}
