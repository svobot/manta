use crate::material::scatter;
use crate::objects::Object;
use crate::ray::Ray;
use crate::spaces::Vec3;
use std::ops::{Add, AddAssign, Mul};

#[derive(Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    pub fn r(&self) -> &f64 {
        &self.r
    }

    pub fn g(&self) -> &f64 {
        &self.g
    }

    pub fn b(&self) -> &f64 {
        &self.b
    }

    pub fn write(&self, samples: i32, pixels: &mut [u8]) {
        // Divide the color by the number of samples and gamma-correct for gamma=2.0.
        let scale = 1. / samples as f64;
        let color_corr = |color: &f64| (scale * color).sqrt();

        // Convert colors to their 8-bit values.
        let fun = |color: &f64| (256. * clamp(color_corr(color), 0., 0.999)) as u8;
        pixels[0] = fun(self.r());
        pixels[1] = fun(self.g());
        pixels[2] = fun(self.b());
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Object, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = scatter(ray, &hit) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        } else {
            return Color::new(0., 0., 0.);
        }
    }

    // Background:
    let t = 0.5 * (ray.direction.y() + 1.);
    let start_color = Color::new(1., 1., 1.);
    let end_color = Color::new(0.5, 0.7, 1.);
    start_color * (1. - t) + end_color * t
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
