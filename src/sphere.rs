use crate::aabb::*;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::rt_weekend::*;
use crate::vec3::Vec3;
use std::sync::Arc;
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Arc<dyn Material>,
}
impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = oc * ray.direction();
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        get_sphere_uv(outward_normal, &mut rec.u, &mut rec.v);
        rec.material = self.material.clone();
        true
    }

    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        );
        true
    }
}
/// # Arguments
///  
/// - `p`: a given point on the sphere of radius one, centered at the origin.
/// - `u`: returned value [0,1] of angle around the Y axis from X=-1.
/// - `v`: returned value [0,1] of angle from Y=-1 to Y=+1.
///
/// # Examples
///
/// >    <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
/// >    <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
/// >    <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
///
pub fn get_sphere_uv(p: Vec3, u: &mut f64, v: &mut f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z() / p.x()).atan() + PI;

    *u = phi / (2. * PI);
    *v = theta / PI;
}
pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}
impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }
    pub fn center(&self, time: f64) -> Vec3 {
        self.center0
            + (self.center1 - self.center0) * (time - self.time0) / (self.time1 - self.time0)
    }
}
impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center(ray.time());
        let a = ray.direction().squared_length();
        let half_b = oc * ray.direction();
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_d = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center(ray.time())) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = self.material.clone();
        true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut AABB) -> bool {
        let box0 = AABB::new(
            self.center(time0) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Vec3::new(self.radius, self.radius, self.radius),
        );
        let box1 = AABB::new(
            self.center(time1) - Vec3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Vec3::new(self.radius, self.radius, self.radius),
        );
        *output_box = surrounding_box(box0, box1);
        true
    }
}
