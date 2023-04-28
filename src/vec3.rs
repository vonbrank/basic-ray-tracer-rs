use std::{f32::EPSILON, ops};

use crate::utils::{random_f32, random_f32_with_range};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: (f32, f32, f32),
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: (x, y, z) }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_f32(), random_f32(), random_f32())
    }

    pub fn random_with_range(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_f32_with_range(min, max),
            random_f32_with_range(min, max),
            random_f32_with_range(min, max),
        )
    }

    pub fn x(&self) -> f32 {
        self.e.0
    }

    pub fn y(&self) -> f32 {
        self.e.1
    }

    pub fn z(&self) -> f32 {
        self.e.2
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
        u.e.0 * v.e.0 + u.e.1 * v.e.1 + u.e.2 * v.e.2
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.e.1 * v.e.2 - u.e.2 * v.e.1,
            u.e.2 * v.e.0 - u.e.0 * v.e.2,
            u.e.0 * v.e.1 - u.e.1 * v.e.0,
        )
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
    pub fn near_zero(&self) -> bool {
        let eps = EPSILON;
        self.e.0.abs() < eps && self.e.1.abs() < eps && self.e.2.abs() < eps
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e.0 += rhs.e.0;
        self.e.1 += rhs.e.1;
        self.e.2 += rhs.e.2;
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.e.0 -= rhs.e.0;
        self.e.1 -= rhs.e.1;
        self.e.2 -= rhs.e.2;
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e.0 *= rhs.e.0;
        self.e.1 *= rhs.e.1;
        self.e.2 *= rhs.e.2;
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e.0 *= rhs;
        self.e.1 *= rhs;
        self.e.2 *= rhs;
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(mut self, rhs: f32) -> Self::Output {
        self *= 1.0 / rhs;
        self
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e.0 /= rhs;
        self.e.1 /= rhs;
        self.e.2 /= rhs;
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_with_range(-1.0, 1.0);
        if p.length_squared() < 0.5 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    Vec3::unit_vector(&random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if (Vec3::dot(&in_unit_sphere, normal) > 0.0) {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = f32::min(Vec3::dot(&-(*uv), n), 1.0);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt() * *n);
    r_out_perp + r_out_parallel
}
