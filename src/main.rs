use image::{png::*, ColorType};
use objects::{ObjectList, Sphere};
use rand::prelude::*;
use std::error::Error;
use std::fs::File;
use std::rc::Rc;
use vec3::BoundVec3;

mod camera;
mod color;
mod objects;
mod ray;
mod vec3;

fn write_image(pixels: &[u8], width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let output = File::create("image.png")?;
    let encoder = PngEncoder::new(output);
    encoder.encode(&pixels, width, height, ColorType::Rgb8)?;
    Ok(())
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = ObjectList {
        objects: vec![
            Rc::new(Sphere::new(BoundVec3::new(0., 0., -1.), 0.5)),
            Rc::new(Sphere::new(BoundVec3::new(0., -100.5, -1.), 100.)),
        ],
    };

    // Camera
    let cam = camera::Camera::new();

    // Render
    let mut rng = thread_rng();
    let mut pixels = vec![0u8; 3 * image_width * image_height];
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = color::Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = cam.ray(u, v);
                pixel_color += color::ray_color(&r, &world, max_depth);
            }
            let pixel_index = 3 * ((image_height - 1 - j) * image_width + i);
            pixel_color.write(samples_per_pixel, &mut pixels[pixel_index..pixel_index + 3]);
        }
    }
    write_image(&pixels, image_width as u32, image_height as u32).expect("error writing PNG file");

    eprintln!("\nDone.");
}
