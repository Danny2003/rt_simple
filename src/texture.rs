use crate::vec3::Vec3;
use std::sync::Arc;
pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3;
}
pub struct SolidColor {
    color_value: Vec3,
}
impl SolidColor {
    pub fn new(color_value: Vec3) -> Self {
        SolidColor { color_value }
    }
    #[allow(dead_code)]
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Vec3 {
        Vec3::new(red, green, blue)
    }
}
impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Vec3) -> Vec3 {
        self.color_value
    }
}
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}
impl CheckerTexture {
    #[allow(dead_code)]
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self { odd, even }
    }
    pub fn new_rgb(even: Vec3, odd: Vec3) -> Self {
        Self {
            odd: Arc::new(SolidColor::new(odd)),
            even: Arc::new(SolidColor::new(even)),
        }
    }
}
impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Vec3) -> Vec3 {
        let sines = (p.x() * 10.).sin() * (p.y() * 10.).sin() * (p.z() * 10.).sin();
        if sines < 0. {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}