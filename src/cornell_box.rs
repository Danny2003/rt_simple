use crate::aabb::AABB;
use crate::aarect::*;
use crate::hit::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;
pub struct CornellBox {
    box_min: Vec3,
    box_max: Vec3,
    sides: HitList,
}
impl CornellBox {
    #[allow(dead_code)]
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> Self {
        let mut sides = HitList::new();
        sides.add(Arc::new(XYRectangle::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p1.z(),
            material.clone(),
        )));
        sides.add(Arc::new(XYRectangle::new(
            p0.x(),
            p1.x(),
            p0.y(),
            p1.y(),
            p0.z(),
            material.clone(),
        )));
        sides.add(Arc::new(XZRectangle::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p1.y(),
            material.clone(),
        )));
        sides.add(Arc::new(XZRectangle::new(
            p0.x(),
            p1.x(),
            p0.z(),
            p1.z(),
            p0.y(),
            material.clone(),
        )));
        sides.add(Arc::new(YZRectangle::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p1.x(),
            material.clone(),
        )));
        sides.add(Arc::new(YZRectangle::new(
            p0.y(),
            p1.y(),
            p0.z(),
            p1.z(),
            p0.x(),
            material.clone(),
        )));
        Self {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}
impl Hittable for CornellBox {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = AABB::new(self.box_min, self.box_max);
        true
    }
}
