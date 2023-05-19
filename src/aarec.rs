use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::Hittable,
    material::{EmptyMaterial, Material},
    vec3::{Point3, Vec3},
};

pub struct XYRect {
    mat: Arc<dyn Material>,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XYRect {
    pub fn default() -> XYRect {
        XYRect {
            mat: Arc::new(EmptyMaterial {}),
            x0: 0.0,
            x1: 0.0,
            y0: 0.0,
            y1: 0.0,
            k: 0.0,
        }
    }
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, mat: Arc<dyn Material>) -> XYRect {
        XYRect {
            mat,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XYRect {
    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut crate::aabb::AABB) -> bool {
        std::mem::swap(
            output_box,
            &mut AABB::new(
                &Point3::new(self.x0, self.y0, self.k),
                &Point3::new(self.x1, self.y1, self.k),
            ),
        );

        true
    }

    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();

        if t < t_min || t > t_max {
            return false;
        }

        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();

        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }

        rec.u = (x - self.x0) / (self.x1 - self.x0);
        rec.v = (y - self.y0) / (self.y1 - self.y0);
        rec.t = t;
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);
        rec.mat = self.mat.clone();
        rec.p = r.at(t);

        true
    }
}
