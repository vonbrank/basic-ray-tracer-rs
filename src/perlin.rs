use crate::{
    utils::{random_f32, random_i32_with_range},
    vec3::Point3,
};

const point_count: usize = 256;
#[derive(Debug)]
pub struct Perlin {
    ranfloat: Vec<f32>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut ranfloat = Vec::with_capacity(point_count);
        for i in 0..point_count {
            ranfloat.push(random_f32());
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let i = (p.x() * 4.0) as i32 & 255;
        let j = (p.y() * 4.0) as i32 & 255;
        let k = (p.z() * 4.0) as i32 & 255;

        self.ranfloat
            [(self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]) as usize]
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(point_count);
        for i in 0..point_count {
            p.push(i as i32);
        }

        Perlin::permute(&mut p, point_count);

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (0..n).rev() {
            let target = random_i32_with_range(0, i as i32) as usize;
            p.swap(i, target);
        }
    }
}
