use std::{fmt::Debug, sync::Arc};

use crate::{
    perlin::Perlin,
    vec3::{Color, Point3},
};
use image::io::Reader as ImageReader;

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

#[derive(Debug)]
pub struct ImageTexture {
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
    data: Vec<u8>,
}

impl ImageTexture {
    const bytes_per_pixel: u32 = 3;

    pub fn default() -> ImageTexture {
        ImageTexture {
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
            data: vec![],
        }
    }

    pub fn new(filename: String) -> ImageTexture {
        let mut image_option: Option<_> = None;

        match ImageReader::open(filename.clone()) {
            Ok(buffer) => match buffer.decode() {
                Ok(image) => {
                    image_option = Some(image);
                }
                _ => {}
            },
            _ => {}
        }

        match image_option {
            Some(image) => ImageTexture {
                width: image.width(),
                height: image.height(),
                bytes_per_scanline: image.width() * ImageTexture::bytes_per_pixel,
                data: image.into_bytes().to_vec(),
            },
            _ => {
                eprintln!("ERROR: Could not load texture image file {}", filename);
                ImageTexture {
                    width: 0,
                    height: 0,
                    bytes_per_scanline: 0,
                    data: vec![],
                }
            }
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, p: &Point3) -> Color {
        if self.data.len() == 0 {
            Color::new(0.0, 1.0, 1.0)
        } else {
            let u = u.clamp(0.0, 1.0);
            let v = 1.0 - v.clamp(0.0, 1.0);

            let mut i = (u * (self.width as f32)) as u32;
            let mut j = (v * (self.height as f32)) as u32;

            if i >= self.width {
                i = self.width - 1;
            }
            if j >= self.height {
                j = self.height - 1;
            }

            let color_scale = 1.0 / 255.0;
            let pixel_index = j * self.bytes_per_scanline + i * ImageTexture::bytes_per_pixel;

            Color::new(
                color_scale * self.data[pixel_index as usize] as f32,
                color_scale * self.data[(pixel_index + 1) as usize] as f32,
                color_scale * self.data[(pixel_index + 2) as usize] as f32,
            )
        }
    }
}
