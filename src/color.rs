use std::{fmt::Display, num};

use crate::vec3::Color;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (256.0 * self.x().clamp(0.0, 0.999)) as i32,
            (256.0 * self.y().clamp(0.0, 0.999)) as i32,
            (256.0 * self.z().clamp(0.0, 0.999)) as i32
        )
    }
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / samples_per_pixel as f32;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();

    println!("{}", Color::new(r, g, b));
}
