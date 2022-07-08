use crate::ray::Ray;
use crate::rt_weekend::*;
use crate::vec3::Vec3;
/// Camera decides the direction of the ray according to the pixel's position.
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}
impl Camera {
    /// The constructor of the Camera.
    /// # Arguments
    ///
    /// * `vfov` - vertical field-of-view in degrees
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = Vec3::unit(look_from - look_at);
        let u = Vec3::unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        Self {
            origin: look_from,
            horizontal: u * viewport_width,
            vertical: v * viewport_height,
            lower_left_corner: look_from - u * viewport_width / 2.0 - v * viewport_height / 2.0 - w,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin,
        }
    }
}
