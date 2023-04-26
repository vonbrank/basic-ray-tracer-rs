use crate::{color::write_color, vec3::Color};

mod color;
mod vec3;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_width = 256;
    let image_height = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let pixel_color: Color = Color::new(
                (i as f32) / (image_width as f32 - 1.0),
                (j as f32) / (image_height as f32 - 1.0),
                0.25,
            );

            write_color(pixel_color);
        }
    }

    Ok(())
}
