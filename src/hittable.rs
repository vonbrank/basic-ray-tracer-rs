use std::{rc::Rc, sync::Arc};

use crate::{
    material::{EmptyMaterial, Material},
    ray::Ray,
    vec3::{Point3, Vec3},
};
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            t: 1.0,
            front_face: true,
            mat: Arc::new(EmptyMaterial {}),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable: Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
