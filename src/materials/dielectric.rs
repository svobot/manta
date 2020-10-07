use super::Material;
use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::{reflection, refraction};
use rand::Rng;

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let refract_ratio = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = (-ray.direction).dot(&hit.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refract_ratio * sin_theta > 1.;
        let direction = if cannot_refract || reflectance(cos_theta, refract_ratio) > rand::random()
        {
            reflection(ray.direction.into(), hit.normal.into())
        } else {
            refraction(ray.direction, hit.normal, refract_ratio)
        };
        Some((Color::new(1., 1., 1.), Ray::new(&hit.p, &direction.into())))
    }
}

fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    // Use Schlick's reflectance approximation.
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cos).powi(5)
}
