use crate::aarec::{XYRect, XZRect, YZRect};
use crate::bvh::BvhNode;
use crate::constant_medium::ConstantMedium;
use crate::cube::Cube;
use crate::hittable::{HitRecord, Hittable, RotateY, Translate};
use crate::material::DiffuseLight;
use crate::moving_sphere::MovingSphere;
use crate::ray::Ray;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor};
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

pub fn ray_color(r: &Ray, background: &Color, world: Arc<dyn Hittable>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if !world.hit(r, EPSILON * 9e4, f32::MAX, &mut rec) {
        return background.clone();
    }

    let mut scattered = Ray::default();
    let mut attenuation = Color::default();
    let emitted = rec.mat.emitted(rec.u, rec.v, &rec.p);

    if !rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
        return emitted;
    }

    emitted + attenuation * ray_color(&scattered, background, world, depth - 1)
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
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material.clone(),
                    )));
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

pub fn two_shpheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::with_color(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));

    objects
}

pub fn two_perlin_shpheres() -> HittableList {
    let mut objects = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));

    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));

    objects
}

pub fn hittable_list_earth() -> HittableList {
    let mut objects = HittableList::new();

    let image_texture = Arc::new(ImageTexture::new("assets/earthmap.jpg".to_string()));
    let earth_surface = Arc::new(Lambertian::new(image_texture));

    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    )));

    objects
}

pub fn hittable_list_simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(perlin_texture.clone())),
    )));

    let diffuse_light = Arc::new(DiffuseLight::with_color(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diffuse_light.clone(),
    )));

    objects
}

pub fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::with_color(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::with_color(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::with_color(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::with_color(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        light.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let cube1 = Arc::new(Cube::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let cube1 = Arc::new(RotateY::new(cube1, 15.0));
    let cube1 = Arc::new(Translate::new(cube1, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(cube1);

    let cube2 = Arc::new(Cube::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let cube2 = Arc::new(RotateY::new(cube2, -18.0));
    let cube2 = Arc::new(Translate::new(cube2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.add(cube2);

    objects
}

pub fn cornell_box_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::with_color(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::with_color(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::with_color(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::with_color(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        green.clone(),
    )));
    objects.add(Arc::new(YZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        red.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        light.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let cube1 = Arc::new(Cube::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    let cube1 = Arc::new(RotateY::new(cube1, 15.0));
    let cube1 = Arc::new(Translate::new(cube1, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(Arc::new(ConstantMedium::with_color(
        cube1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    let cube2 = Arc::new(Cube::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    ));
    let cube2 = Arc::new(RotateY::new(cube2, -18.0));
    let cube2 = Arc::new(Translate::new(cube2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.add(Arc::new(ConstantMedium::with_color(
        cube2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    objects
}

pub fn hittalbe_list_final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::with_color(&Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_f32_with_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(Cube::new(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Arc::new(BvhNode::with_hittable_list(&boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::with_color(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let cneter1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = cneter1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::with_color(&Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        cneter1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::with_center_and_radius(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::with_color(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::with_center_and_radius(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::with_color(
        boundary.clone(),
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let earth_material = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
        "assets/earthmap.jpg".to_string(),
    ))));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));
    let perlin_texture = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::with_center_and_radius(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(perlin_texture)),
    )));

    let mut boxes2 = HittableList::new();

    let white = Arc::new(Lambertian::with_color(&Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::with_center_and_radius(
            Point3::random_with_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::with_hittable_list(&boxes2, 0.0, 1.0)),
            15.0,
        )),
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
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
