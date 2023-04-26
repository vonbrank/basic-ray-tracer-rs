use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new() -> Ray {
        Ray {
            orig: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn with_origin_and_direction(origin: &Point3, direction: &Point3) -> Ray {
        Ray {
            orig: *origin,
            dir: *direction,
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
}
