use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Vec3,      // hitting point
    pub normal: Vec3, // the normal direction of the hitting surface
    pub t: f64,       // hitting time
    pub front_face: bool,
}
impl HitRecord {
    pub fn new(p: Vec3, normal: Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
    pub fn zero() -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
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
// Box<dyn Hittable> is a trait object, which is a pointer to a dynamically allocated object.
pub struct HitList {
    pub list: Vec<Box<dyn Hittable>>,
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
        let mut temp_rec = HitRecord::zero();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.list {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }
        hit_anything
    }
}
