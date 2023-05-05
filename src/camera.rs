use crate::{
    ray::Ray,
    utils::{degrees_to_radians, random_f32_with_range},
    vec3::{random_in_unit_disk, Point3, Vec3},
};

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    basis: (Vec3, Vec3, Vec3),
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        look_from: &Point3,
        look_at: &Point3,
        vup: &Point3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time0: f32,
        time1: f32,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(*look_from - *look_at));
        let u = Vec3::unit_vector(&Vec3::cross(vup, &w));
        let v = Vec3::cross(&w, &u);

        let origin = look_from.clone();
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
            basis: (u, v, w),
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.basis.0 * rd.x() + self.basis.1 * rd.y();

        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset),
            random_f32_with_range(self.time0, self.time1),
        )
    }
}
