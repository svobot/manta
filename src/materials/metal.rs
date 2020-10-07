use super::Material;
use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vec3::{FreeVec3, UnitVec3};

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
        let reflection = |v: FreeVec3, n: FreeVec3| v - n * v.dot(&n) * 2.;
        let scattered = Ray::new(
            &hit.p,
            &UnitVec3::from(
                reflection(ray.direction.into(), hit.normal.into())
                    + FreeVec3::from(UnitVec3::random_unit_vector()) * self.fuzziness,
            ),
        );
        if scattered.direction.dot(&hit.normal) > 0. {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
