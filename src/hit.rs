use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
#[derive(Clone)]
pub struct HitRecord {
    /// hitting point
    pub p: Vec3,
    /// the normal direction of the hitting surface
    pub normal: Vec3,
    /// ---------------------
    /// We'll use shared pointers in our code,
    /// because it allows multiple geometries to share a common instance
    /// (for example, a bunch of spheres that all use the same texture map material),
    /// and because it makes memory management automatic and easier to reason about.
    /// ---------------------
    /// the material of the hitting surface
    pub material: Arc<dyn Material>,
    /// hitting time
    pub t: f64,
    /// whether the hit happens on the front face of the hitting surface
    pub front_face: bool,
}
impl HitRecord {
    /// impl a default constructor for HitRecord
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: true,
            material,
        }
    }
    pub fn set_face_normal(&mut self, r: Ray, outward_normal: Vec3) {
        self.front_face = (r.direction() * outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
pub struct HitList {
    /// Box<dyn Hittable> is a trait object, which is a pointer to a dynamically allocated object.
    pub list: Vec<Box<dyn Hittable>>,
}
impl Default for HitList {
    fn default() -> Self {
        Self::new()
    }
}
impl HitList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.list.push(object);
    }
    pub fn clear(&mut self) {
        self.list.clear();
    }
}
impl Hittable for HitList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = rec.clone();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in self.list.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
