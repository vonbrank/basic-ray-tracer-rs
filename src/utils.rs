use rand::{self, Rng};
use std::{
    f32::consts::PI,
    io::{self, Write},
    time::Duration,
};

use crate::vec3::Color;

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
    eprint!("\x1B[1F");
    eprint!("\x1B[0G");
    eprint!("\x1B[0J");
}
