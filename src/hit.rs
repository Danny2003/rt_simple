use crate::aabb::*;
use crate::material::*;
use crate::ray::Ray;
use crate::rt_weekend::*;
use crate::vec3::Vec3;
use std::f64::INFINITY;
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
    /// the U,V surface coordinates of the ray-object hit point
    pub u: f64,
    pub v: f64,
    /// whether the hit happens on the front face of the hitting surface
    pub front_face: bool,
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            material: Arc::new(Lambertian::new(Vec3::zero())),
            t: 0.,
            u: 0.,
            v: 0.,
            front_face: true,
        }
    }
}
impl HitRecord {
    /// impl a default constructor for HitRecord
    pub fn new(material: Arc<dyn Material>) -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.,
            front_face: true,
            material,
            u: 0.,
            v: 0.,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = (r.direction() * outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool;
}
#[derive(Clone)]
pub struct HitList {
    /// Box<dyn Hittable> is a trait object, which is a pointer to a dynamically allocated object.
    pub list: Vec<Arc<dyn Hittable>>,
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
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.list.push(object);
    }
    pub fn clear(&mut self) {
        self.list.clear();
    }
}
impl Hittable for HitList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
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
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        if self.list.is_empty() {
            return false;
        }
        let mut temp_box: AABB = Default::default();
        let mut first_box = true;

        for object in self.list.iter() {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                surrounding_box(*output_box, temp_box)
            };
            first_box = false;
        }
        true
    }
}
pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}
impl Translate {
    pub fn new(ptr: Arc<dyn Hittable>, offset: Vec3) -> Self {
        Self { ptr, offset }
    }
}
impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        if !self.ptr.hit(&moved_ray, t_min, t_max, rec) {
            return false;
        }
        rec.p += self.offset;
        rec.set_face_normal(&moved_ray, rec.normal);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let mut temp_box: AABB = Default::default();
        if !self.ptr.bounding_box(time0, time1, &mut temp_box) {
            return false;
        }
        *output_box = AABB::new(temp_box.min() + self.offset, temp_box.max() + self.offset);
        true
    }
}
pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    has_box: bool,
    bbox: AABB,
}
impl RotateY {
    pub fn new(ptr: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox: AABB = Default::default();
        let has_box = ptr.bounding_box(0., 1., &mut bbox);

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().x() + (1 - i) as f64 * bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1 - j) as f64 * bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1 - k) as f64 * bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let mut tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        *min.get(c) = min.get(c).min(*tester.get(c));
                        *max.get(c) = max.get(c).max(*tester.get(c));
                    }
                }
            }
        }
        bbox = AABB::new(min, max);
        Self {
            ptr,
            sin_theta,
            cos_theta,
            has_box,
            bbox,
        }
    }
}
impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time11: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        self.has_box
    }
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        *origin.get(0) =
            *ray.origin().get(0) * self.cos_theta - *ray.origin().get(2) * self.sin_theta;
        *origin.get(2) =
            *ray.origin().get(0) * self.sin_theta + *ray.origin().get(2) * self.cos_theta;

        *direction.get(0) =
            *ray.direction().get(0) * self.cos_theta - *ray.direction().get(2) * self.sin_theta;
        *direction.get(2) =
            *ray.direction().get(0) * self.sin_theta + *ray.direction().get(2) * self.cos_theta;

        let rotated_r = Ray::new(origin, direction, ray.time());

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }
        let mut p = rec.p;
        let mut normal = rec.normal;

        *p.get(0) = *rec.p.get(0) * self.cos_theta + *rec.p.get(2) * self.sin_theta;
        *p.get(2) = -*rec.p.get(0) * self.sin_theta + *rec.p.get(2) * self.cos_theta;

        *normal.get(0) = *rec.normal.get(0) * self.cos_theta + *rec.normal.get(2) * self.sin_theta;
        *normal.get(2) = -*rec.normal.get(0) * self.sin_theta + *rec.normal.get(2) * self.cos_theta;

        rec.p = p;
        rec.set_face_normal(&rotated_r, normal);

        true
    }
}
