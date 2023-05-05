use std::sync::Arc;

use crate::{
    hittable::Hittable,
    material::Material,
    sphere::Sphere,
    vec3::{Point3, Vec3},
};

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f32,
    time1: f32,
    sphere: Sphere,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f32,
        time1: f32,
        radius: f32,
        m: Arc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            sphere: Sphere::with_center_and_radius(center0, radius, m),
        }
    }

    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + (time - self.time0) / (self.time1 - self.time0) * (self.center1 - self.center0)
    }

    pub fn radius(&self) -> f32 {
        self.sphere.radius()
    }
    pub fn mat(&self) -> Arc<dyn Material> {
        self.sphere.mat()
    }
}

impl Hittable for MovingSphere {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let ac = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = Vec3::dot(&r.direction(), &ac);
        let c = ac.length_squared() - self.radius() * self.radius();
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
        let outward_normal = (rec.p - self.center(r.time())) / self.radius();
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Arc::clone(&self.mat());

        true
    }
}
