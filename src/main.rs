use std::{f32::EPSILON, rc::Rc};

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{random_in_unit_sphere, Vec3};

use crate::{
    camera::Camera,
    color::write_color,
    hittable_list::HittableList,
    spere::Sphere,
    utils::random_f32,
    vec3::{Color, Point3},
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod spere;
mod utils;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, EPSILON, f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5
            * ray_color(
                &Ray::with_origin_and_direction(&rec.p, &(target - rec.p)),
                world,
                depth - 1,
            );
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    let aspect_ratio = 16.0_f32 / 9.0_f32;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Rc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera

    let cam = Camera::new();

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_f32()) / (image_width as f32 - 1.0);
                let v = (j as f32 + random_f32()) / (image_height as f32 - 1.0);
                let ray = cam.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    Ok(())
}
