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
    pub fn new(vfov: f64, aspect_ratio: f64) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        // We make it the z = −2 plane here.
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        Self {
            origin: Vec3::zero(),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
            lower_left_corner: Vec3::new(
                -viewport_width / 2.0,
                -viewport_height / 2.0,
                -focal_length,
            ),
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}
