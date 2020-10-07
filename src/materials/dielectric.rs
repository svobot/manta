use super::Material;
use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::UnitVec3;

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

        let refract = |uv: UnitVec3, n: UnitVec3, etai_over_etat: f64| {
            let cos_theta = (-uv).dot(&n);
            let r_out_perp = (n * cos_theta + uv.into()) * etai_over_etat;
            let r_out_parallel = n * -(1. - r_out_perp.length_squared()).abs().sqrt();
            r_out_perp + r_out_parallel
        };

        let refracted = refract(ray.direction, hit.normal, refract_ratio);
        Some((Color::new(1., 1., 1.), Ray::new(&hit.p, &refracted.into())))
    }
}
