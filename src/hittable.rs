use std::sync::Arc;

use crate::{
    aabb::AABB,
    material::{EmptyMaterial, Material},
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};
#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
    pub u: f32,
    pub v: f32,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(1.0, 0.0, 0.0),
            t: 1.0,
            front_face: true,
            mat: Arc::new(EmptyMaterial {}),
            u: 0.0,
            v: 0.0,
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

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool;
}

pub struct Translate {
    ptr: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable>, displacement: &Vec3) -> Translate {
        Translate {
            ptr: p,
            offset: displacement.clone(),
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let moved_ray = Ray::new(&(r.origin() - self.offset), &r.direction(), r.time());

        if !self.ptr.hit(&moved_ray, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        let rec_normal = rec.normal;
        rec.set_face_normal(&moved_ray, &rec_normal);

        true
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        std::mem::swap(
            output_box,
            &mut (AABB::new(
                &(output_box.min() + self.offset),
                &(output_box.max() + self.offset),
            )),
        );

        true
    }
}

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    has_box: bool,
    aabb_box: AABB,
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable>, angle: f32) -> RotateY {
        let radians = degrees_to_radians(angle);

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut aabb_box = AABB::default();
        let has_box = p.bounding_box(0.0, 1.0, &mut aabb_box);

        let mut min_point = Point3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max_point = Point3::new(-f32::MAX, -f32::MAX, -f32::MAX);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * aabb_box.max().x() + (1 - i) as f32 * aabb_box.min().x();
                    let y = j as f32 * aabb_box.max().y() + (1 - j) as f32 * aabb_box.min().y();
                    let z = k as f32 * aabb_box.max().z() + (1 - k) as f32 * aabb_box.min().z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min_point[c] = f32::min(min_point[c], tester[c]);
                        max_point[c] = f32::max(max_point[c], tester[c]);
                    }
                }
            }
        }

        let aabb_box = AABB::new(&min_point, &max_point);

        RotateY {
            ptr: p,
            sin_theta,
            cos_theta,
            has_box,
            aabb_box,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin()[0] - self.sin_theta * r.origin()[2];
        origin[2] = self.sin_theta * r.origin()[0] + self.cos_theta * r.origin()[2];

        direction[0] = self.cos_theta * r.direction()[0] - self.sin_theta * r.direction()[2];
        direction[2] = self.sin_theta * r.direction()[0] + self.cos_theta * r.direction()[2];

        let rotated_ray = Ray::new(&origin, &direction, r.time());

        if !self.ptr.hit(&rotated_ray, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p.clone();
        let mut normal = rec.normal.clone();

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_ray, &normal);

        true
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut AABB) -> bool {
        std::mem::swap(output_box, &mut (self.aabb_box.clone()));
        self.has_box
    }
}
