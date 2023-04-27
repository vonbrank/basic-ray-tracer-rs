use std::ops;

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
