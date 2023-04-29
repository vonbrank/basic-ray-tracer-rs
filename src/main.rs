use hittable::{HitRecord, Hittable};
use material::{EmptyMaterial, Material};
use ray::Ray;
use std::{
    f32::EPSILON,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
use vec3::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector, Vec3};

use crate::{
    camera::Camera,
    color::to_color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    thread_pool::ThreadPool,
    utils::{clean_screen, format_duration_hhmmss, random_f32, PixelInfo},
    vec3::{Color, Point3},
};

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod sphere;
mod thread_pool;
mod utils;
mod vec3;

fn ray_color(r: &Ray, world: Arc<HittableList>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.hit(r, EPSILON * 9e4, f32::MAX, &mut rec) {
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

fn print_progress(current_pixel: usize, total_pixels: usize, current_duration: Duration) {
    let progress_bar_length = 25;
    let percentage = (current_pixel as f32 / total_pixels as f32 * 100.0) as i32;
    let progress = ((percentage as f32 / 100.0) * progress_bar_length as f32) as i32;
    clean_screen();
    eprint!(
        "Rendering: [{}{}] {}%\n{}   {} pixel(s) rendered",
        "#".repeat(progress as usize),
        "-".repeat((progress_bar_length - progress) as usize),
        percentage,
        format_duration_hhmmss(current_duration),
        current_pixel,
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1600;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let world = Arc::new(random_scene());

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
    let pool = ThreadPool::new(num_threads);
    let (sender, receiver): (SyncSender<PixelInfo>, Receiver<PixelInfo>) = sync_channel(16);
    let sender = Arc::new(sender);

    let handle = thread::spawn(move || {
        let total_pixels = image_width * image_height;
        let mut buffer =
            vec![vec![Color::new(1.0, 1.0, 1.0); image_width as usize]; image_height as usize];

        let start_time = Instant::now();
        let mut last_print_time = start_time.clone();

        for i in 0..total_pixels {
            let pixel_info = receiver.recv().unwrap();
            buffer[pixel_info.v][pixel_info.u] = pixel_info.color;

            let current_time = Instant::now();
            let elapsed_time = current_time.duration_since(last_print_time);
            if elapsed_time >= Duration::from_secs(1) || i == total_pixels - 1 {
                last_print_time = current_time;
                print_progress(i + 1, total_pixels, current_time - start_time);
            }
        }
        for j in (0..image_height).rev() {
            for i in 0..image_width {
                println!("{}", buffer[j][i]);
            }
        }
    });

    for j in (0..image_height).rev() {
        let arc_world = Arc::clone(&world);
        let arc_sender = Arc::clone(&sender);
        pool.execute(move || {
            for i in 0..image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..samples_per_pixel {
                    let u = (i as f32 + random_f32()) / (image_width as f32 - 1.0);
                    let v = (j as f32 + random_f32()) / (image_height as f32 - 1.0);
                    let ray = cam.get_ray(u, v);
                    pixel_color += ray_color(&ray, arc_world.clone(), max_depth);
                }
                pixel_color = to_color(pixel_color, samples_per_pixel);
                arc_sender
                    .send(PixelInfo {
                        u: i,
                        v: j as usize,
                        color: pixel_color,
                    })
                    .unwrap();
            }
        });
    }

    if let Err(msg) = handle.join() {
        eprintln!("\n{:?}", msg);
    }

    Ok(())
}
