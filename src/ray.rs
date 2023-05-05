use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub tm: f32,
}

impl Ray {
    pub fn default() -> Ray {
        Ray {
            orig: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 1.0, 1.0),
            tm: 0.0,
        }
    }

    pub fn new(origin: &Point3, direction: &Point3, time: f32) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn time(&self) -> f32 {
        self.tm
    }
}
