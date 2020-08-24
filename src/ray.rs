use crate::vec3::{BoundVec3, UnitVec3};

pub struct Ray {
    pub origin: BoundVec3,
    pub direction: UnitVec3,
}

impl Ray {
    pub fn new(origin: &BoundVec3, direction: &UnitVec3) -> Self {
        Ray {
            origin: (*origin).clone(),
            direction: (*direction).clone(),
        }
    }

    pub fn at(&self, t: f64) -> BoundVec3 {
        self.origin + self.direction * t
    }
}
