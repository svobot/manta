use super::Material;
use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::FreeVec3;

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let reflection = |v: FreeVec3, n: FreeVec3| v - n * v.dot(&n) * 2.;
        let scattered = Ray::new(
            &hit.p,
            &reflection(ray.direction.into(), hit.normal.into()).into(),
        );
        if FreeVec3::from(scattered.direction).dot(&hit.normal.into()) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
