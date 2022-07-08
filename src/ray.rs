use crate::vec3::Vec3;
#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}
impl Ray {
    pub fn origin(&self) -> Vec3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }
    pub fn zero() -> Self {
        Self {
            orig: Vec3::zero(),
            dir: Vec3::zero(),
        }
    }
    pub fn at(self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
