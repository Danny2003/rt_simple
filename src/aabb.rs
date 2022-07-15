use crate::ray::Ray;
use crate::vec3::Vec3;
#[derive(Clone, Debug, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub struct AABB {
    min: Vec3,
    max: Vec3,
}
impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }
    pub fn min(&self) -> Vec3 {
        self.min
    }
    pub fn max(&self) -> Vec3 {
        self.max
    }
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_dir = 1.0 / *r.direction().get(a);
            let mut t0 = (*self.min().get(a) - *r.origin().get(a)) * inv_dir;
            let mut t1 = (*self.max().get(a) - *r.origin().get(a)) * inv_dir;
            if inv_dir < 0. {
                (t0, t1) = (t1, t0);
            }
            t_min = t0.max(t_min);
            t_max = t1.min(t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}
impl Default for AABB {
    fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::zero())
    }
}
pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let min = Vec3::new(
        box0.min().x().min(box1.min().x()),
        box0.min().y().min(box1.min().y()),
        box0.min().z().min(box1.min().z()),
    );
    let max = Vec3::new(
        box0.max().x().max(box1.max().x()),
        box0.max().y().max(box1.max().y()),
        box0.max().z().max(box1.max().z()),
    );
    AABB { min, max }
}
