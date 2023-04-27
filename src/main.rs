use ray::Ray;
use vec3::Vec3;

use crate::{
    color::write_color,
    vec3::{Color, Point3},
};

mod color;
mod ray;
mod vec3;
mod hittable;
mod spere;

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal: Vec3 = Vec3::unit_vector(&(r.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f32, r: &Ray) -> f32 {
    let ac = r.origin() - *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let half_b = Vec3::dot(&r.direction(), &ac);
    let c = Vec3::dot(&ac, &ac) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant > 0.0 {
        (-half_b - discriminant.sqrt()) / a
    } else {
        -1.0
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    let aspect_ratio = 16.0_f32 / 9.0_f32;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

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
            let pixel_color = ray_color(&ray);

            write_color(pixel_color);
        }
    }

    Ok(())
}
