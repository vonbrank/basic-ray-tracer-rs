use crate::{
    utils::{random_f32, random_i32_with_range},
    vec3::{Point3, Vec3},
};
#[derive(Debug)]
pub struct Perlin {
    rand_vec: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const point_count: usize = 256;

    pub fn new() -> Perlin {
        let mut rand_vec = Vec::with_capacity(Perlin::point_count);
        for i in 0..Perlin::point_count {
            rand_vec.push(Vec3::random_with_range(-1.0, 1.0));
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f32 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize]
                }
            }
        }

        Perlin::trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = Vec::with_capacity(Perlin::point_count);
        for i in 0..Perlin::point_count {
            p.push(i as i32);
        }

        Perlin::permute(&mut p, Perlin::point_count);

        p
    }

    fn permute(p: &mut Vec<i32>, n: usize) {
        for i in (0..n).rev() {
            let target = random_i32_with_range(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f32, v - j as f32, w - k as f32);
                    accum += (i as f32 * uu + (1 - i) as f32 * (1.0 - uu))
                        * (j as f32 * vv + (1 - j) as f32 * (1.0 - vv))
                        * (k as f32 * ww + (1 - k) as f32 * (1.0 - ww))
                        * Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }
        accum
    }
}
