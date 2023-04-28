use std::{fmt::Debug, rc::Rc};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{random_in_unit_sphere, random_unit_vector, reflect, Color, Vec3},
};

pub trait Material: Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}
#[derive(Debug, Clone)]
pub struct EmptyMaterial {}

impl Material for EmptyMaterial {}

#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    fn new() -> Lambertian {
        Lambertian {
            albedo: Color::new(0.0, 0.0, 0.0),
        }
    }
    pub fn with_albedo(a: &Color) -> Lambertian {
        Lambertian { albedo: *a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        std::mem::swap(
            scattered,
            &mut Ray::with_origin_and_direction(&rec.p, &scatter_direction),
        );
        std::mem::swap(attenuation, &mut self.albedo.clone());
        true
    }
}
#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(a: &Color, f: f32) -> Metal {
        Metal {
            albedo: *a,
            fuzz: if f < 1.0 { f } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&Vec3::unit_vector(&r_in.direction()), &rec.normal);
        std::mem::swap(
            scattered,
            &mut Ray::with_origin_and_direction(
                &rec.p,
                &(reflected + self.fuzz * random_in_unit_sphere()),
            ),
        );
        std::mem::swap(attenuation, &mut self.albedo.clone());
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
