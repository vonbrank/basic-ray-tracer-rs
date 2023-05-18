use std::{fmt::Debug, sync::Arc};

use crate::{
    perlin::Perlin,
    vec3::{Color, Point3},
};

pub trait Texture: Debug + Send + Sync {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color;
}
#[derive(Debug)]
pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn default() -> SolidColor {
        SolidColor {
            color_value: Color::new(0.0, 0.0, 0.0),
        }
    }

    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.color_value
    }
}
#[derive(Debug)]
pub struct CheckerTexture {
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }

    pub fn with_color(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        let sines = f32::sin(10.0 * p.x()) * f32::sin(10.0 * p.y()) * f32::sin(10.0 * p.z());

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn default() -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }

    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f32::sin(self.scale * p.z() + 10.0 * self.noise.turb(p, 7)))
    }
}
