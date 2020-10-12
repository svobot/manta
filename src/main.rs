use color::Color;
use image::{png::*, ColorType};
use material::Material;
use objects::{ObjectList, Sphere};
use rand::prelude::*;
use spaces::{FreeVec3, Point, Vec3};
use std::error::Error;
use std::fs::File;
use std::rc::Rc;

mod camera;
mod color;
mod material;
mod objects;
mod ray;
mod spaces;

fn write_image(pixels: &[u8], width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let output = File::create("image.png")?;
    let encoder = PngEncoder::new(output);
    encoder.encode(&pixels, width, height, ColorType::Rgb8)?;
    Ok(())
}

fn random_scene() -> ObjectList {
    let mut world = ObjectList {
        objects: Vec::new(),
    };
    // Ground
    let material_ground = Material::Lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        material_ground,
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point::new(
                0.9 * rng.gen::<f64>() + (a as f64),
                0.2,
                0.9 * rng.gen::<f64>() + (b as f64),
            );

            if (center - Point::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let c1 = Color::new(rng.gen(), rng.gen(), rng.gen());
                    let c2 = Color::new(rng.gen(), rng.gen(), rng.gen());

                    let albedo = c1 * c2;
                    let sphere_material = Material::Lambertian(albedo);
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::new(
                        rng.gen_range(0.5, 1.),
                        rng.gen_range(0.5, 1.),
                        rng.gen_range(0.5, 1.),
                    );
                    let fuzziness = rng.gen_range(0., 0.5);
                    let sphere_material = Material::Metal {
                        color: albedo,
                        fuzziness,
                    };
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // Glass
                    let sphere_material = Material::Dielectric(1.5);
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }
    let material1 = Material::Dielectric(1.5);
    world.add(Rc::new(Sphere::new(Point::new(0., 1., 0.), 1., material1)));

    let material2 = Material::Lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Rc::new(Sphere::new(Point::new(-4., 1., 0.), 1., material2)));
    let material3 = Material::Metal {
        color: Color::new(0.7, 0.6, 0.5),
        fuzziness: 0.,
    };
    world.add(Rc::new(Sphere::new(Point::new(4., 1., 0.), 1., material3)));

    world
}

fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let world = random_scene();

    // Camera
    let lookfrom = Point::new(13., 2., 3.);
    let lookat = FreeVec3::new(0., 0., 0.);
    let cam = camera::Camera::new(
        lookfrom,
        lookat,
        FreeVec3::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.1,
        10.,
    );

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
