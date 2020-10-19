use super::{HitRecord, Object};
use crate::ray::Ray;
use std::sync::Arc;

pub struct ObjectList {
    pub objects: Vec<Arc<dyn Object + Send + Sync>>,
}

impl ObjectList {
    pub fn add(&mut self, object: Arc<dyn Object + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Object for ObjectList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut closest_hit = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
