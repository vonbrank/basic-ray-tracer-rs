use std::{fmt::Debug, rc::Rc, sync::Arc};

use crate::{
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    utils::random_f32,
    vec3::{random_in_unit_sphere, random_unit_vector, reflect, refract, Color, Vec3},
};

pub trait Material: Debug + Send + Sync {
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
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    // fn new() -> Lambertian {
    //     Lambertian {
    //         albedo: Color::new(0.0, 0.0, 0.0),
    //     }
    // }

    pub fn with_color(a: &Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(a.clone())),
        }
    }

    pub fn new(a: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: a }
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
            &mut Ray::new(&rec.p, &scatter_direction, r_in.time()),
        );
        std::mem::swap(attenuation, &mut self.albedo.value(rec.u, rec.v, &rec.p));
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
            &mut Ray::new(
                &rec.p,
                &(reflected + self.fuzz * random_in_unit_sphere()),
                r_in.time(),
            ),
        );
        std::mem::swap(attenuation, &mut self.albedo.clone());
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Dielectric {
        Dielectric {
            ir: index_of_refraction,
        }
    }

    fn reflectance(cosine: f32, ref_index: f32) -> f32 {
        let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        std::mem::swap(attenuation, &mut Color::new(1.0, 1.0, 1.0));

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = Vec3::unit_vector(&r_in.direction());

        let cos_theta = f32::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        std::mem::swap(scattered, &mut Ray::new(&rec.p, &direction, r_in.time()));
        true
    }
}
