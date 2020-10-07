use super::{HitRecord, Object};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::BoundVec3;
use std::rc::Rc;

pub struct Sphere {
    center: BoundVec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: BoundVec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction.into());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            for &t in [(-half_b - root) / a, (-half_b + root) / a].iter() {
                if t < t_max && t > t_min {
                    let p = ray.at(t);
                    return Some(HitRecord::new(
                        ray,
                        p,
                        ((p - self.center) / self.radius).into(),
                        t,
                        self.material.clone(),
                    ));
                }
            }
        }

        None
    }
}
