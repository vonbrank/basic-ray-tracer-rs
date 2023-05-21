use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    texture::{SolidColor, Texture},
    utils::random_f32,
    vec3::{random_in_unit_sphere, Color, Vec3},
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn with_texture(b: Arc<dyn Hittable>, d: f32, a: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Isotropic::with_texture(a)),
            neg_inv_density: -1.0 / d,
        }
    }
    pub fn with_color(b: Arc<dyn Hittable>, d: f32, c: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Isotropic::with_color(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && random_f32() < 0.00001;

        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();

        if !self.boundary.hit(r, -f32::MAX, f32::MAX, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.0001, f32::MAX, &mut rec2) {
            return false;
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }

        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_f32().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = self.phase_function.clone();

        true
    }

    fn bounding_box(&self, time0: f32, time1: f32, output_box: &mut crate::aabb::AABB) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
}
#[derive(Debug)]
struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn with_color(c: Color) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }

    pub fn with_texture(a: Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo: a }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        std::mem::swap(attenuation, &mut self.albedo.value(rec.u, rec.v, &rec.p));
        std::mem::swap(
            scattered,
            &mut (Ray::new(&rec.p, &random_in_unit_sphere(), r_in.time())),
        );

        true
    }
}
