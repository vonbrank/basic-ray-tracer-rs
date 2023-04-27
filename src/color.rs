use std::{fmt::Display, num};

use crate::vec3::Color;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            256.0 * self.x().clamp(0.0, 0.999),
            256.0 * self.y().clamp(0.0, 0.999),
            256.0 * self.z().clamp(0.0, 0.999)
        )
    }
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r *= scale;
    g *= scale;
    b *= scale;

    println!("{}", Color::new(r, g, b));
}
