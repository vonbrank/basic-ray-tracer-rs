use std::{rc::Rc, sync::Arc};

use crate::{
    hittable::Hittable,
    material::{EmptyMaterial, Material},
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Point3,
    radius: f32,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            mat: Arc::new(EmptyMaterial {}),
        }
    }

    pub fn with_center_and_radius(center: Point3, radius: f32, m: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let ac = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&r.direction(), &ac);
        let c = ac.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Arc::clone(&self.mat);

        true
    }
}

unsafe impl Send for Sphere {}
