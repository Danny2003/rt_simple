use crate::{rt_weekend::*, vec3::Vec3};
static POINT_COUNT: usize = 256;
/// A key part of Perlin noise is that it is repeatable:
/// it takes a 3D point as input and always returns the same randomish number.
/// Nearby points return similar numbers. Another important part of Perlin noise is that
/// it be simple and fast, so it’s usually done as a hack.
/// I’ll build that hack up incrementally based on Andrew Kensler’s description.
///
/// We could just tile all of space with a 3D array of random numbers and use them in blocks. You get something blocky where the repeating is clear:
/// ![Image 6: Tiled random patterns](https://raytracing.github.io/images/img-2.06-tile-random.jpg)
/// Let’s just use some sort of hashing to scramble this, instead of tiling. This has a bit of support code to make it all happen:
pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}
impl Perlin {
    pub fn new() -> Self {
        let mut ran_vec = vec![Vec3::zero(); POINT_COUNT];
        for item in ran_vec.iter_mut().take(POINT_COUNT) {
            *item = Vec3::unit(Vec3::random_in_range(-1., 1.));
        }
        Self {
            ran_vec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    /// Smoothing yields an improved result, but there are obvious grid features in there.
    /// Some of it is Mach bands, a known perceptual artifact of linear interpolation of color.
    /// A standard trick is to use a Hermite cubic to round off the interpolation:
    pub fn noise(&self, p: &Vec3) -> f64 {
        // // 按位与取后 8 位
        // let i = ((p.x() * 4.) as i32 & 255) as usize;
        // let j = ((p.y() * 4.) as i32 & 255) as usize;
        // let k = ((p.z() * 4.) as i32 & 255) as usize;
        // // 按位异或
        // self.ran_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = vec![[[Vec3::zero(); 2]; 2]; 2];
        #[allow(clippy::needless_range_loop)]
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ran_vec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }
    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = vec![0; POINT_COUNT];

        for (i, item) in p.iter_mut().enumerate().take(POINT_COUNT) {
            *item = i as i32;
        }
        Self::permute(&mut p, POINT_COUNT);
        p
    }
    fn permute(p: &mut [i32], n: usize) {
        for i in (1..n).rev() {
            let target = random_int_in_range(0, i as i32) as usize;
            p.swap(i, target);
        }
    }
    #[allow(dead_code)]
    fn trilinear_interp(c: Vec<[[f64; 2]; 2]>, u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.;
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1. - i as f64) * (1. - u))
                        * (j as f64 * v + (1. - j as f64) * (1. - v as f64))
                        * (k as f64 * w as f64 + (1. - k as f64) * (1. - w as f64))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
    fn perlin_interp(c: Vec<[[Vec3; 2]; 2]>, u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3. - 2. * u);
        let vv = v * v * (3. - 2. * v);
        let ww = w * w * (3. - 2. * w);
        let mut accum = 0.;
        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1. - i as f64) * (1. - uu))
                        * (j as f64 * vv + (1. - j as f64) * (1. - vv as f64))
                        * (k as f64 * ww as f64 + (1. - k as f64) * (1. - ww as f64))
                        * (c[i][j][k] * weight_v);
                }
            }
        }
        accum
    }
}
