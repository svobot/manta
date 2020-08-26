use std::rc::Rc;
use vec3::{BoundVec3, FreeVec3};

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

    let world = hittable_list::HittableList {
        hittables: vec![
            Rc::new(sphere::Sphere::new(BoundVec3::new(0., 0., -1.), 0.5)),
            Rc::new(sphere::Sphere::new(BoundVec3::new(0., -100.5, -1.), 100.)),
        ],
    };

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = BoundVec3::new(0.0, 0.0, 0.0);
    let horizontal = FreeVec3::new(viewport_width, 0.0, 0.0);
    let vertical = FreeVec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - FreeVec3::new(0.0, 0.0, focal_length);

    print!(
        "P3\n{width} {height}\n255\n",
        width = image_width,
        height = image_height
    );
    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = ray::Ray::new(
                &origin,
                &(lower_left_corner + horizontal * u + vertical * v - origin).into(),
            );

            let pixel_color = color::ray_color(&r, &world);
            println!("{}", &pixel_color);
        }
    }

    eprintln!("\nDone.");
}
