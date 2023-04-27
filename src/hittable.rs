use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
