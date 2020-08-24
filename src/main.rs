mod color;
mod ray;
mod vec3;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = vec3::BoundVec3::new(0.0, 0.0, 0.0);
    let horizontal = vec3::FreeVec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::FreeVec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3::FreeVec3::new(0.0, 0.0, focal_length);

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

            let pixel_color = color::ray_color(&r);
            println!("{}", &pixel_color);
        }
    }

    eprintln!("\nDone.");
}
