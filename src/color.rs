use std::fmt::Display;

use crate::vec3::Color;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            255.999 * self.x(),
            255.999 * self.y(),
            255.999 * self.z()
        )
    }
}

pub fn write_color(pixel_color: Color) {
    println!("{}", pixel_color);
}
