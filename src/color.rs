use crate::ray::Ray;
use crate::vec3::*;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    fn new(r: f64, g: f64, b: f64) -> Self {
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
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.r()) as i32,
            (255.999 * self.g()) as i32,
            (255.999 * self.b()) as i32
        )
    }
}

pub fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(BoundVec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let t = 0.5 * (ray.direction.y() + 1.0);
    let start_color = Color::new(1.0, 1.0, 1.0);
    let end_color = Color::new(0.5, 0.7, 1.0);
    start_color * (1.0 - t) + end_color * t
}

fn hit_sphere(center: BoundVec3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - center;
    let a = r.direction.free().dot(&r.direction.free());
    let b = 2.0 * oc.dot(&r.direction.free());
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
