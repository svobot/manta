use crate::spaces::{Point, UnitVec3};

pub struct Ray {
    pub origin: Point,
    pub direction: UnitVec3,
}

impl Ray {
    pub fn new(origin: &Point, direction: &UnitVec3) -> Self {
        Ray {
            origin: (*origin).clone(),
            direction: (*direction).clone(),
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
