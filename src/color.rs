use crate::vec3::Vec3;
use std::{fs::File, io::Write};
/// clamps the value x to the range [min,max]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
/// the multi-sample write_color() function
pub fn write_color(pixel_color: Vec3, file: &mut File, samples_per_pixel: u32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    // Write the translated [0,255] value of each color component.
    file.write_all(
        ((256.0 * clamp(r, 0.0, 0.999)).floor() as i32)
            .to_string()
            .as_bytes(),
    )
    .expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all(
        ((256.0 * clamp(g, 0.0, 0.999)).floor() as i32)
            .to_string()
            .as_bytes(),
    )
    .expect("wrong write");
    file.write_all(b" ").expect("wrong write");
    file.write_all(
        ((256.0 * clamp(b, 0.0, 0.999)).floor() as i32)
            .to_string()
            .as_bytes(),
    )
    .expect("wrong write");
    file.write_all(b"\n").expect("wrong write");
}
