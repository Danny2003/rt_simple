use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}
impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(
        &self,
        // unused variable (if this is intentional, prefix it with an underscore) according to the warning
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
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}
// Metal material with reflectance function
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
        );
        *attenuation = self.albedo;
        // if the scattered ray is below the surface, return false
        scattered.direction() * rec.normal > 0.0
    }
}
