use super::Material;
use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::spaces::vec3::reflection;
use crate::spaces::{UnitVec3, Vec3};

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new(
            &hit.p,
            &UnitVec3::from(
                reflection(ray.direction, hit.normal)
                    + UnitVec3::random_unit_vector() * self.fuzziness,
            ),
        );
        if scattered.direction.dot(&hit.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
