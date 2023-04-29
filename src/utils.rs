use rand::{self, Rng};
use std::f32::consts::PI;

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
