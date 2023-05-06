use crate::hittable::{HitRecord, Hittable};
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::texture::{CheckerTexture, SolidColor};
use crate::vec3::Vec3;
use rand::{self, Rng};
use std::{
    f32::consts::PI,
    f32::EPSILON,
    io::{self, Write},
    sync::Arc,
    time::Duration,
};

use crate::{
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::{Color, Point3},
};

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

pub fn random_f32_with_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_f32()
}

pub fn random_i32_with_range(min: i32, max: i32) -> i32 {
    random_f32_with_range(min as f32, (max + 1) as f32) as i32
}

pub struct PixelInfo {
    pub u: usize,
    pub v: usize,
    pub color: Color,
}

pub fn format_duration_hhmmss(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

pub fn clean_screen() {
    io::stderr().flush().unwrap();
    eprint!("\x1B[3F");
    eprint!("\x1B[0G");
    eprint!("\x1B[0J");
}

pub fn ray_color(r: &Ray, world: Arc<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, EPSILON * 9e4, f32::MAX, &mut rec) {
        let mut scattered = Ray::default();
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

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new(checker));
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
                if choose_mat < 0.7 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material =
                        Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo))));
                    let center2 = center + Vec3::new(0.0, random_f32_with_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::with_center_and_radius(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                    // world.add(Arc::new(MovingSphere::new(
                    //     center,
                    //     center2,
                    //     0.0,
                    //     1.0,
                    //     0.2,
                    //     sphere_material.clone(),
                    // )));
                } else if choose_mat < 0.9 {
                    let albedo = Color::random_with_range(0.5, 1.0);
                    let sphere_material =
                        Arc::new(Lambertian::new(Arc::new(SolidColor::new(albedo))));
                    world.add(Arc::new(Sphere::with_center_and_radius(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::with_center_and_radius(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                    if random_f32() < 0.5 {
                        world.add(Arc::new(Sphere::with_center_and_radius(
                            center,
                            -0.15,
                            sphere_material.clone(),
                        )));
                    }
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));
    world.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 1.0, 0.0),
        -0.9,
        material1.clone(),
    )));
    let material2 = Arc::new(Lambertian::with_color(&Color::new(0.4, 0.2, 0.1)));
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

pub fn print_progress(
    current_pixel: usize,
    total_pixels: usize,
    current_duration: Duration,
    num_cpu_threads: usize,
    pixels_per_second: usize,
) {
    let progress_bar_length = 25;
    let percentage = (current_pixel as f32 / total_pixels as f32 * 100.0) as i32;
    let progress = ((percentage as f32 / 100.0) * progress_bar_length as f32) as i32;
    eprintln!(
        "Rendering:  [{}{}] {}%",
        "#".repeat(progress as usize),
        "-".repeat((progress_bar_length - progress) as usize),
        percentage,
    );
    eprintln!(
        "{}    {} pixel(s) per second",
        format_duration_hhmmss(current_duration),
        pixels_per_second
    );
    eprintln!(
        "threads={}  {} pixel(s) rendered",
        num_cpu_threads, current_pixel,
    );
}
