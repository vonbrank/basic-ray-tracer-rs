use std::rc::Rc;

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::Vec3;

use crate::{
    color::write_color,
    hittable_list::HittableList,
    spere::Sphere,
    vec3::{Color, Point3},
};

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod spere;
mod utils;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::new();
    if world.hit(r, 0.0, f32::MAX, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
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

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = (i as f32) / (image_width as f32 - 1.0);
            let v = (j as f32) / (image_height as f32 - 1.0);
            let ray = Ray::with_origin_and_direction(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&ray, &world);

            write_color(pixel_color);
        }
    }

    Ok(())
}
