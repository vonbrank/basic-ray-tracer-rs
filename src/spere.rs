use crate::{
    hittable::Hittable,
    vec3::{Point3, Vec3},
};

struct Sphere {
    center: Point3,
    radius: f32,
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
        let a = Vec3::dot(&r.direction(), &r.direction());
        let half_b = Vec3::dot(&r.direction(), &ac);
        let c = Vec3::dot(&ac, &ac) - self.radius * self.radius;
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
        rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}
