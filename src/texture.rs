use crate::color::clamp;
use crate::perlin::*;
use crate::vec3::Vec3;
use image::*;
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
/// We can create a checker texture by noting that
/// the sign of sine and cosine just alternates in a regular way,
/// and if we multiply trig functions in all three dimensions,
/// the sign of that product forms a 3D checker pattern.
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
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}
impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Vec3) -> Vec3 {
        // Noise texture with marbled texture
        let n = 0.5 * (1. + (self.scale * p.z() + 10. * self.noise.turb(p, 7)).sin());
        // let n = self.noise.turb(&(*p * self.scale), 7);
        // let n = 0.5 * (1.0 + self.noise.noise(&(*p * self.scale)));
        Vec3::new(n, n, n)
    }
}
pub struct ImageTexture {
    data: ImageBuffer<Rgb<u8>, Vec<u8>>,
    width: usize,
    height: usize,
}
impl Default for ImageTexture {
    fn default() -> Self {
        Self {
            data: ImageBuffer::new(0, 0),
            width: 0,
            height: 0,
        }
    }
}
impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let data = open(filename).unwrap().into_rgb8();
        let width = data.width() as usize;
        let height = data.height() as usize;
        Self {
            data,
            width,
            height,
        }
    }
}
impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Vec3) -> Vec3 {
        // Clamp input texture coordinates to [0,1] x [1,0]
        u = clamp(u, 0., 1.);
        v = 1.0 - clamp(v, 0., 1.); // Flip V to image coordinates
        let mut i = (u * self.width as f64).floor() as usize;
        let mut j = (v * self.height as f64).floor() as usize;
        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }
        let pixel = self.data.get_pixel(i as u32, j as u32).to_rgb();
        Vec3::new(
            pixel[0] as f64 / 255.,
            pixel[1] as f64 / 255.,
            pixel[2] as f64 / 255.,
        )
    }
}
