use std::{
    f32::{consts::PI, EPSILON},
    sync::{Arc, Mutex},
    thread,
};

use hittable::{HitRecord, Hittable};
use material::{EmptyMaterial, Material};
use ray::Ray;
use vec3::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector, Vec3};

use crate::{
    camera::Camera,
    color::{to_color, write_color},
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

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::with_albedo(&Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point3::new(
                a as f32 + 0.9 * random_f32(),
                0.2,
                b as f32 + 0.9 * random_f32(),
            );

            if (center - Point3::new(4.0, 0.2, 0.9)).length() > 0.9 {
                let mut sphere_material: Arc<dyn Material> = Arc::new(EmptyMaterial {});

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::with_albedo(&albedo));
                    world.add(Arc::new(Sphere::with_center_and_radius(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_with_range(0.5, 1.0);
                    sphere_material = Arc::new(Lambertian::with_albedo(&albedo));
                    world.add(Arc::new(Sphere::with_center_and_radius(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::with_center_and_radius(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Arc::new(Lambertian::with_albedo(&Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}

fn print_progress(current_row: i32, max_height: i32) {
    let progress_bar_length = 25;
    let percentage = (current_row as f32 / max_height as f32 * 100.0) as i32;
    let progress = ((percentage as f32 / 100.0) * progress_bar_length as f32) as i32;
    eprint!(
        "\x1B[2K\x1B[1GRendering: [{}{}] {}% row-{}",
        "#".repeat(progress as usize),
        "-".repeat((progress_bar_length - progress) as usize),
        percentage,
        current_row,
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1600;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let world = random_scene();

    // Camera

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

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

    let num_threads = 16;
    let buffer = Arc::new(Mutex::new(vec![
        vec![
            Color::new(1.0, 1.0, 1.0);
            image_width as usize
        ];
        image_height as usize
    ]));

    let completed_count = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..num_threads)
        .map(|thread_num: i32| {
            let start_row = (thread_num as i32 * image_height / num_threads as i32) as usize;
            let end_row = ((thread_num as i32 + 1) * image_height / num_threads as i32) as usize;
            let local_world = Box::new(world.clone());
            let loacl_buffer_mutext = buffer.clone();
            let local_completed_count = completed_count.clone();
            let handle = thread::spawn(move || {
                for j in (start_row..end_row).rev() {
                    for i in 0..image_width {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                        for _ in 0..samples_per_pixel {
                            let u = (i as f32 + random_f32()) / (image_width as f32 - 1.0);
                            let v = (j as f32 + random_f32()) / (image_height as f32 - 1.0);
                            let ray = cam.get_ray(u, v);
                            pixel_color += ray_color(&ray, &local_world, max_depth);
                        }
                        let mut current_buffer = loacl_buffer_mutext.lock().unwrap();
                        current_buffer[j][i] = to_color(pixel_color, samples_per_pixel);
                    }
                    let mut val = local_completed_count.lock().unwrap();
                    *val += 1;
                    eprintln!("row {} completed at thread {}", val, thread_num);
                }
            });
            handle
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let current_buffer = buffer.lock().unwrap();

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            println!("{}", current_buffer[j as usize][i]);
        }
    }

    Ok(())
}
