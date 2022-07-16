use crate::aabb::*;
use crate::hit::*;
use crate::rt_weekend::*;
use crate::Ray;
use std::cmp::Ordering;
use std::sync::Arc;
use std::vec::Vec;
#[derive(Clone)]
pub struct BVHNode {
    /// The bounding box of this node.
    pub bbox: AABB,
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
}
impl BVHNode {
    pub fn new(mut objects: Vec<Arc<dyn Hittable>>, time0: f64, time1: f64) -> Self {
        // Create a modifiable array of the source scene objects
        let axis = random_int_in_range(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        }; // Choose a random axis to sort along
        let object_span = objects.len();
        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;
        if object_span == 1 {
            // If there is only one object, return it as the left and right child.
            left = objects.pop().unwrap();
            right = left.clone();
        } else if object_span == 2 {
            // If there are two objects, return them as the left and right child.
            if comparator(objects.first().unwrap(), objects.last().unwrap()) == Ordering::Less {
                right = objects.pop().unwrap();
                left = objects.pop().unwrap();
            } else {
                left = objects.pop().unwrap();
                right = objects.pop().unwrap();
            }
        } else {
            // If there are more than two objects, sort the objects along the chosen axis.
            objects.sort_by(comparator);
            // Create the left and right child nodes.
            let mut left_vec = objects;
            let right_vec = left_vec.split_off(object_span / 2);
            left = Arc::new(Self::new(left_vec, time0, time1));
            right = Arc::new(Self::new(right_vec, time0, time1));
        }
        let mut box_left: AABB = Default::default();
        let mut box_right: AABB = Default::default();

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            println!("No bounding box in BVHNode constructor.\n");
        }
        // Calculate the bounding box of the left and right child nodes.
        let bbox = surrounding_box(box_left, box_right);
        Self { bbox, left, right }
    }
}
impl Hittable for BVHNode {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        true
    }
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        // record the earliest hit time
        let hit_right = self
            .right
            .hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
}
pub fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: i32) -> Ordering {
    let mut box_a: AABB = Default::default();
    let mut box_b: AABB = Default::default();

    if !a.bounding_box(0., 0., &mut box_a) || !b.bounding_box(0., 0., &mut box_b) {
        println!("No bounding box in BVHNode constructor.\n");
    }
    box_a.min().get(axis).total_cmp(box_b.min().get(axis))
}
pub fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
