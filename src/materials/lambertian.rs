use super::Material;
use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::{FreeVec3, UnitVec3};

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_dir = FreeVec3::from(hit.normal) + UnitVec3::random_unit_vector().into();
        Some((self.albedo, Ray::new(&hit.p, &scatter_dir.into())))
    }
}
