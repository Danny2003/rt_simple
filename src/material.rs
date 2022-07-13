use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::rt_weekend::*;
use crate::texture::*;
use crate::vec3::Vec3;
use std::sync::Arc;
/// utility function to compare f64 values
pub fn fmin(a: f64, b: f64) -> f64 {
    if a > b {
        b
    } else {
        a
    }
}
pub trait Material: Sync + Send {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}
/// We can make textured materials by replacing the `Vec3` a with a texture pointer:
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}
impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        Self {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }
    pub fn new_texture(a: Arc<dyn Texture>) -> Self {
        Self { albedo: a }
    }
}
impl Material for Lambertian {
    /// # Arguments
    /// * `_r_in` - an unused variable (if this is intentional, prefix it with an underscore) according to the warning
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction, _r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
/// Metal material with reflectance function
pub struct Metal {
    albedo: Vec3,
    fuzzy: f64,
}
impl Metal {
    pub fn new(albedo: Vec3, fuzzy: f64) -> Self {
        Self {
            albedo,
            fuzzy: if fuzzy < 1.0 { fuzzy } else { 1.0 },
        }
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit(r_in.direction()), &rec.normal);
        *scattered = Ray::new(
            rec.p,
            reflected + Vec3::random_in_unit_sphere() * self.fuzzy,
            r_in.time(),
        );
        *attenuation = self.albedo;
        // if the scattered ray is below the surface, return false, which leads to the "absorption" of the light
        scattered.direction() * rec.normal > 0.0
    }
}
/// Dielectric material class that always refracts when possible
///
/// # Approximation
///
/// Now real glass has reflectivity that varies with angle — look at a window at a steep angle and it becomes a mirror.
/// There is a big ugly equation for that,
/// but almost everybody uses a cheap and surprisingly accurate polynomial approximation by Christophe Schlick.
/// This yields our full glass material.
///
/// # a Hollow Glass Sphere
///
/// An interesting and easy trick with dielectric spheres is
/// to note that if you use a negative radius,
/// the geometry is unaffected, but the surface normal points inward.
/// This can be used as a bubble to make a hollow glass sphere
///
pub struct Dielectric {
    /// Index of Refraction
    ref_idx: f64,
}
impl Dielectric {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::ones();
        let refraction_ratio = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = Vec3::unit(r_in.direction());
        let cos_theta = fmin(-unit_direction * rec.normal, 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        // If "cannot_refract", all the light is reflected,
        // and because in practice that is usually inside solid objects, it is called “total internal reflection”.
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random_double() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };
        *scattered = Ray::new(rec.p, direction, r_in.time());
        true
    }
}
