use super::{HitRecord, Object};
use crate::material::Material;
use crate::ray::Ray;
use crate::spaces::{Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Material) -> Self {
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
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

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
