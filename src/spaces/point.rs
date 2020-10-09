use super::vec3::{FreeVec3, Vec3};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
}

impl<T: Vec3> Add<T> for Point {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        Point::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl<T: Vec3> Sub<T> for Point {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        Point::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl Sub for Point {
    type Output = FreeVec3;

    fn sub(self, rhs: Point) -> Self::Output {
        FreeVec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Vec3> AddAssign<T> for Point {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.x();
        self.y += rhs.y();
        self.z += rhs.z();
    }
}

impl SubAssign<FreeVec3> for Point {
    fn sub_assign(&mut self, rhs: FreeVec3) {
        self.x -= rhs.x();
        self.y -= rhs.y();
        self.z -= rhs.z();
    }
}
