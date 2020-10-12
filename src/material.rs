use crate::color::Color;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::spaces::vec3::{reflection, refraction};
use crate::spaces::{FreeVec3, UnitVec3, Vec3};

#[derive(Clone)]
pub enum Material {
    Lambertian(Color),
    Metal { color: Color, fuzziness: f64 },
    Dielectric(f64),
}

pub fn scatter(ray: &Ray, hit: &HitRecord) -> Option<(Color, Ray)> {
    match hit.material {
        Material::Lambertian(color) => {
            let scatter_dir = FreeVec3::from(hit.normal) + UnitVec3::random_unit_vector();
            Some((color, Ray::new(&hit.p, &scatter_dir.into())))
        }
        Material::Metal { color, fuzziness } => {
            let scattered = Ray::new(
                &hit.p,
                &UnitVec3::from(
                    reflection(ray.direction, hit.normal)
                        + UnitVec3::random_unit_vector() * fuzziness,
                ),
            );
            if scattered.direction.dot(&hit.normal) > 0. {
                Some((color, scattered))
            } else {
                None
            }
        }
        Material::Dielectric(refraction_index) => {
            let refract_ratio = if hit.front_face {
                1.0 / refraction_index
            } else {
                refraction_index
            };

            let cos_theta = (-ray.direction).dot(&hit.normal).min(1.);
            let sin_theta = (1. - cos_theta * cos_theta).sqrt();
            let cannot_refract = refract_ratio * sin_theta > 1.;
            let direction =
                if cannot_refract || reflectance(cos_theta, refract_ratio) > rand::random() {
                    reflection(ray.direction, hit.normal)
                } else {
                    refraction(ray.direction, hit.normal, refract_ratio)
                };
            Some((Color::new(1., 1., 1.), Ray::new(&hit.p, &direction.into())))
        }
    }
}

fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    // Use Schlick's reflectance approximation.
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cos).powi(5)
}
