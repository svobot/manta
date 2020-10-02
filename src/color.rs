use crate::hittable::Hittable;
use crate::ray::Ray;
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

    pub fn write(&self, samples: i32) {
        let scale = 1.0 / samples as f64;
        println!(
            "{} {} {}",
            (256.0 * clamp(self.r() * scale, 0.0, 0.999)) as i32,
            (256.0 * clamp(self.g() * scale, 0.0, 0.999)) as i32,
            (256.0 * clamp(self.b() * scale, 0.0, 0.999)) as i32,
        )
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

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

pub fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        return Color::new(
            hit.normal.x() + 1.0,
            hit.normal.y() + 1.0,
            hit.normal.z() + 1.0,
        ) * 0.5;
    }

    // Background:
    let t = 0.5 * (ray.direction.y() + 1.0);
    let start_color = Color::new(1.0, 1.0, 1.0);
    let end_color = Color::new(0.5, 0.7, 1.0);
    start_color * (1.0 - t) + end_color * t
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
