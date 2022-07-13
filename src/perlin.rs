use crate::{rt_weekend::*, vec3::Vec3};
static POINT_COUNT: usize = 256;
pub struct Perlin {
    ran_float: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}
impl Perlin {
    pub fn new() -> Self {
        let mut ran_float = vec![0.; POINT_COUNT];
        for item in ran_float.iter_mut().take(POINT_COUNT) {
            *item = random_double();
        }
        Self {
            ran_float,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }
    pub fn noise(&self, p: &Vec3) -> f64 {
        // 按位与取后 8 位
        let i = ((p.x() * 4.) as i32 & 255) as usize;
        let j = ((p.y() * 4.) as i32 & 255) as usize;
        let k = ((p.z() * 4.) as i32 & 255) as usize;
        // 按位异或
        self.ran_float[(self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize]
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
}
