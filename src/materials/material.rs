use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)>;
}
