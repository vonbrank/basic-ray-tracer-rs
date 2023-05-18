use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};
use vec3::Vec3;

use crate::{
    bvh::BvhNode,
    camera::Camera,
    color::{format_color, to_color},
    hittable::Hittable,
    hittable_list::HittableList,
    thread_pool::ThreadPool,
    utils::{
        clean_screen, print_progress, random_f32, random_scene, ray_color, two_shpheres, PixelInfo, two_perlin_shpheres, hittable_list_earth,
    },
    vec3::{Color, Point3},
};

mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod sphere;
mod texture;
mod thread_pool;
mod utils;
mod vec3;
mod perlin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world: Arc<dyn Hittable> = Arc::new(HittableList::new());
    let mut look_from = Point3::default();
    let mut look_at = Point3::default();
    let mut vfov = 4.0;
    let mut aperture = 0.0;

    let world_type = 0;

    match world_type {
        1 => {
            world = Arc::new(BvhNode::with_hittable_list(&random_scene(), 0.0, 1.0));
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
            aperture = 0.1;
        }
        2 => {
            world = Arc::new(BvhNode::with_hittable_list(&two_shpheres(), 0.0, 1.0));
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        3 => {
            world = Arc::new(BvhNode::with_hittable_list(&two_perlin_shpheres(), 0.0, 1.0));
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
        _ => {
            world = Arc::new(BvhNode::with_hittable_list(&hittable_list_earth(), 0.0, 1.0));
            look_from = Point3::new(13.0, 2.0, 3.0);
            look_at = Point3::new(0.0, 0.0, 0.0);
            vfov = 20.0;
        }
    }

    // Camera

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        &look_from,
        &look_at,
        &vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Render

    print!("P3\n{} {}\n255\n", image_width, image_height);

    let num_threads = num_cpus::get();
    let pool = ThreadPool::new(num_threads);
    let (sender, receiver): (SyncSender<PixelInfo>, Receiver<PixelInfo>) = sync_channel(16);
    let sender = Arc::new(sender);

    let handle = thread::spawn(move || {
        let total_pixels = image_width * image_height;
        let mut buffer =
            vec![vec![Color::new(1.0, 1.0, 1.0); image_width as usize]; image_height as usize];

        let start_time = Instant::now();
        let mut last_print_time = start_time.clone();
        let mut last_index = 0;
        let mut first_print = true;

        for i in 0..total_pixels {
            let pixel_info = receiver.recv().unwrap();
            buffer[pixel_info.v][pixel_info.u] = pixel_info.color;

            let current_time = Instant::now();
            let elapsed_time = current_time.duration_since(last_print_time);
            if elapsed_time >= Duration::from_secs(1) || i == total_pixels - 1 {
                last_print_time = current_time;
                if first_print {
                    first_print = false;
                } else {
                    clean_screen();
                }
                print_progress(
                    i + 1,
                    total_pixels,
                    current_time - start_time,
                    num_threads,
                    i - last_index,
                );
                last_index = i;
            }
        }
        for j in (0..image_height).rev() {
            for i in 0..image_width {
                println!("{}", format_color(&buffer[j][i]));
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
