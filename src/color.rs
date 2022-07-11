use crate::vec3::Vec3;
use image::RgbImage;
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
pub fn write_color(pixel_color: Vec3, samples_per_pixel: u32, img: &mut RgbImage, i: u32, j: u32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();

    let pixel = img.get_pixel_mut(i, j);
    *pixel = image::Rgb([
        (256.0 * clamp(r, 0.0, 0.999)).floor() as u8,
        (256.0 * clamp(g, 0.0, 0.999)).floor() as u8,
        (256.0 * clamp(b, 0.0, 0.999)).floor() as u8,
    ]);
    // Write the translated [0,255] value of each color component.
}
