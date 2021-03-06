use crate::material::Material;
use crate::ray::Ray;
use crate::spaces::{Point, UnitVec3, Vec3};

pub struct HitRecord {
    pub p: Point,
    pub normal: UnitVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(ray: &Ray, p: Point, outward_normal: UnitVec3, t: f64, material: Material) -> Self {
        let front_face = 0. > ray.direction.dot(&outward_normal);
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
            material,
        }
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
