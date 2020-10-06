use crate::ray::Ray;
use crate::vec3::{BoundVec3, FreeVec3, UnitVec3};

pub struct HitRecord {
    pub p: BoundVec3,
    pub normal: UnitVec3,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: BoundVec3, outward_normal: UnitVec3, t: f64) -> Self {
        let front_face = 0. > FreeVec3::from(ray.direction).dot(&outward_normal.into());
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
