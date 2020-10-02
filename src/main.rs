use rand::prelude::*;
use std::rc::Rc;
use vec3::BoundVec3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    let world = hittable_list::HittableList {
        hittables: vec![
            Rc::new(sphere::Sphere::new(BoundVec3::new(0., 0., -1.), 0.5)),
            Rc::new(sphere::Sphere::new(BoundVec3::new(0., -100.5, -1.), 100.)),
        ],
    };

    let cam = camera::Camera::new();
    let mut rng = thread_rng();

    print!(
        "P3\n{width} {height}\n255\n",
        width = image_width,
        height = image_height
    );
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = ((i as f64) + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = cam.ray(u, v);
                pixel_color += color::ray_color(&r, &world);
            }
            pixel_color.write(samples_per_pixel);
        }
    }

    eprintln!("\nDone.");
}
