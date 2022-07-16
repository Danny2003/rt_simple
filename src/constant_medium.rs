use crate::hit::*;
use crate::rt_weekend::*;
use crate::{aabb::AABB, material::*, texture::Texture, Ray, Vec3};
use std::f64::consts::E;
use std::f64::INFINITY;
use std::f64::NEG_INFINITY;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}
impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, color: Vec3) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new(color)),
            neg_inv_density: -1. / density,
        }
    }
    #[allow(dead_code)]
    pub fn new_texture(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary,
            phase_function: Arc::new(Isotropic::new_texture(texture)),
            neg_inv_density: -1. / density,
        }
    }
}
impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
    /// As the ray passes through the volume, it may scatter at any point.
    /// The denser the volume, the more likely that is.
    /// The probability that the ray scatters in any small distance ΔL is:
    /// $$probability = C \cdot \delta L$$
    /// where C is proportional to the optical density of the volume.
    /// If you go through all the differential equations,
    /// for a random number you get a distance where the scattering occurs.
    /// This implement assumes that once a ray exits the constant medium boundary,
    /// it will continue forever outside the boundary
    /// So this particular implementation will work for boundaries like boxes or spheres,
    /// but will not work with toruses or shapes that contain voids
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        const ENABLE_DEBUG: bool = false;
        let debugging: bool = ENABLE_DEBUG && random_double() < 0.00001;

        let mut rec1: HitRecord = Default::default();
        let mut rec2: HitRecord = Default::default();

        if !self.boundary.hit(ray, NEG_INFINITY, INFINITY, &mut rec1) {
            return false;
        }
        if !self.boundary.hit(ray, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }
        if debugging {
            println!("\nt_min={}, t_max={}", rec1.t, rec2.t);
        }
        if rec1.t < t_min {
            rec1.t = t_min
        };
        if rec2.t > t_max {
            rec2.t = t_max
        };

        if rec1.t >= rec2.t {
            return false;
        };

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        let ray_length: f64 = ray.direction().length();
        let distance_inside_boundary: f64 = (rec2.t - rec1.t) * ray_length;

        let hit_distance: f64 = self.neg_inv_density * random_double().log(E);
        // If that distance is outside the volume, then there is no "hit"
        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = ray.at(rec.t);

        if debugging {
            println!(
                "hit_distance = {}\nrec.t = {}\nrec.p = {:?}",
                hit_distance, rec.t, rec.p
            );
        }

        rec.normal = Vec3::new(1., 0., 0.); // arbitrary
        rec.front_face = true; // also arbitrary
        rec.material = self.phase_function.clone();

        true
    }
}
