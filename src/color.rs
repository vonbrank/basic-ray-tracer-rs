use std::{fmt::Display, num};

use crate::vec3::Color;

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    println!("{}", format_color(&Color::new(r, g, b)));
}

pub fn to_color(pixel_color: Color, samples_per_pixel: i32) -> Color {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    Color::new(r, g, b)
}

pub fn format_color(color: &Color) -> String {
    format!(
        "{} {} {}",
        (256.0 * color.x().clamp(0.0, 0.999)) as i32,
        (256.0 * color.y().clamp(0.0, 0.999)) as i32,
        (256.0 * color.z().clamp(0.0, 0.999)) as i32
    )
}
