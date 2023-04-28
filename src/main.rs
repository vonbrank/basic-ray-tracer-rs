use std::{
    f32::{consts::PI, EPSILON},
    rc::Rc,
};

use hittable::{HitRecord, Hittable};
use ray::Ray;
use vec3::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector, Vec3};

use crate::{
    camera::Camera,
    color::write_color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    utils::random_f32,
    vec3::{Color, Point3},
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, EPSILON * 100.0, f32::MAX, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new(0.0, 0.0, 0.0);
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = Vec3::unit_vector(&r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn print_progress(current_row: i32, max_height: i32) {
    let progress_bar_length = 25;
    let percentage = (current_row as f32 / max_height as f32 * 100.0) as i32;
    let progress = ((percentage as f32 / 100.0) * progress_bar_length as f32) as i32;
    eprint!(
        "\x1B[2K\x1B[1GLoading: [{}{}] {}% row-{}",
        "#".repeat(progress as usize),
        "-".repeat((progress_bar_length - progress) as usize),
        percentage,
        current_row,
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    let aspect_ratio = 16.0_f32 / 9.0_f32;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let r = f32::cos(PI / 4.0);
    let mut world = HittableList::new();

    // let material_left = Rc::new(Lambertian::with_albedo(&Color::new(0.0, 0.0, 1.0)));
    // let material_right = Rc::new(Lambertian::with_albedo(&Color::new(1.0, 0.0, 0.0)));

    // world.add(Rc::new(Sphere::with_center_and_radius(
    //     Point3::new(-r, 0.0, -1.0),
    //     r,
    //     material_left,
    // )));
    // world.add(Rc::new(Sphere::with_center_and_radius(
    //     Point3::new(r, 0.0, -1.0),
    //     r,
    //     material_right,
    // )));

    let material_ground = Rc::new(Lambertian::with_albedo(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::with_albedo(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::with_center_and_radius(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::with_center_and_radius(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::with_center_and_radius(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera

    let look_from = Point3::new(3.0, 3.0, 2.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;

    let cam = Camera::new(
        &look_from,
        &look_at,
        &vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        print_progress(image_height - j, image_height);

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
