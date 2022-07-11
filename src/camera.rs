use crate::ray::Ray;
use crate::rt_weekend::*;
use crate::vec3::Vec3;
/// Camera decides the direction of the ray according to the pixel's position.
#[allow(dead_code)]
pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}
impl Camera {
    /// The constructor of the Camera.
    /// # Arguments
    ///
    /// * `vfov` - vertical field-of-view in degrees
    /// * `aperture` - aperture's radius of the camera
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        _time0: f64,
        _time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = Vec3::unit(look_from - look_at);
        let u = Vec3::unit(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        Self {
            w,
            u,
            v,
            origin: look_from,
            horizontal: u * viewport_width * focus_dist,
            vertical: v * viewport_height * focus_dist,
            lower_left_corner: look_from
                - u * viewport_width / 2.0 * focus_dist
                - v * viewport_height / 2.0 * focus_dist
                - w * focus_dist,
            lens_radius: aperture / 2.,
            time0: _time0,
            time1: _time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            time: random_double_in_range(self.time0, self.time1),
        }
    }
}
