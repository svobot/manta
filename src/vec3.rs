use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoundVec3(Vec3);

impl BoundVec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        BoundVec3(Vec3 { x, y, z })
    }
}

impl Add<FreeVec3> for BoundVec3 {
    type Output = Self;

    fn add(self, rhs: FreeVec3) -> Self::Output {
        BoundVec3(self.0 + rhs.0)
    }
}

impl Sub<FreeVec3> for BoundVec3 {
    type Output = Self;

    fn sub(self, rhs: FreeVec3) -> Self::Output {
        BoundVec3(self.0 - rhs.0)
    }
}

impl Sub for BoundVec3 {
    type Output = FreeVec3;

    fn sub(self, rhs: BoundVec3) -> Self::Output {
        FreeVec3(self.0 - rhs.0)
    }
}

impl AddAssign<FreeVec3> for BoundVec3 {
    fn add_assign(&mut self, rhs: FreeVec3) {
        self.0 += rhs.0;
    }
}

impl SubAssign<FreeVec3> for BoundVec3 {
    fn sub_assign(&mut self, rhs: FreeVec3) {
        *self += -rhs;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FreeVec3(Vec3);

impl FreeVec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        FreeVec3(Vec3 { x, y, z })
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.0.x * other.0.x + self.0.y * other.0.y + self.0.z * other.0.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        FreeVec3(Vec3 {
            x: self.0.y * other.0.z - self.0.z * other.0.y,
            y: self.0.z * other.0.x - self.0.x * other.0.z,
            z: self.0.x * other.0.y - self.0.y * other.0.x,
        })
    }

    pub fn length_squared(&self) -> f64 {
        self.0.length_squared()
    }
}

impl From<UnitVec3> for FreeVec3 {
    fn from(v: UnitVec3) -> Self {
        FreeVec3(v.0)
    }
}

impl Neg for FreeVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FreeVec3(Vec3 {
            x: -self.0.x,
            y: -self.0.y,
            z: -self.0.z,
        })
    }
}

impl Mul<f64> for FreeVec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        FreeVec3(self.0 * rhs)
    }
}

impl Div<f64> for FreeVec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        FreeVec3(self.0 / rhs)
    }
}

impl AddAssign<FreeVec3> for FreeVec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl SubAssign<FreeVec3> for FreeVec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl MulAssign<f64> for FreeVec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
    }
}

impl DivAssign<f64> for FreeVec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0.x /= rhs;
        self.0.y /= rhs;
        self.0.z /= rhs;
    }
}

#[derive(Copy, Clone)]
pub struct UnitVec3(Vec3);

impl UnitVec3 {
    pub fn x(&self) -> &f64 {
        &self.0.x
    }

    pub fn y(&self) -> &f64 {
        &self.0.y
    }

    pub fn z(&self) -> &f64 {
        &self.0.z
    }

    pub fn length_squared(&self) -> f64 {
        self.0.length_squared()
    }

    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0., 2. * std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1., 1.);
        let r = (1. - z * z).sqrt();
        FreeVec3::new(r * a.cos(), r * a.sin(), z).into()
    }
}

impl From<FreeVec3> for UnitVec3 {
    fn from(v: FreeVec3) -> Self {
        UnitVec3((v / v.0.length()).0)
    }
}

impl Neg for UnitVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        UnitVec3(Vec3 {
            x: -self.0.x,
            y: -self.0.y,
            z: -self.0.z,
        })
    }
}

impl Mul<f64> for UnitVec3 {
    type Output = FreeVec3;

    fn mul(self, rhs: f64) -> Self::Output {
        FreeVec3(self.0 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bound_arithm() {
        assert_eq!(
            BoundVec3::new(3.0, 3.0, 3.0) + FreeVec3::new(0.0, 1.0, 2.0),
            BoundVec3::new(3.0, 4.0, 5.0)
        );
        assert_eq!(
            BoundVec3::new(3.0, 3.0, 3.0) - FreeVec3::new(0.0, 1.0, 2.0),
            BoundVec3::new(3.0, 2.0, 1.0)
        );
        assert_eq!(
            BoundVec3::new(3.0, 3.0, 3.0) - BoundVec3::new(0.0, 1.0, 2.0),
            FreeVec3::new(3.0, 2.0, 1.0)
        );
    }

    #[test]
    fn free_arithm() {
        let mut a1 = FreeVec3::new(0.0, 1.0, 2.0);
        a1 += FreeVec3::new(0.0, 1.0, 2.0);
        assert_eq!(a1, FreeVec3::new(0.0, 2.0, 4.0));

        let mut s1 = FreeVec3::new(0.0, 1.0, 2.0);
        s1 -= FreeVec3::new(0.0, 1.0, 2.0);
        assert_eq!(s1, FreeVec3::new(0.0, 0.0, 0.0));

        let mut m1 = FreeVec3::new(0.0, 1.0, 2.0);
        m1 *= 2.0;
        assert_eq!(m1, FreeVec3::new(0.0, 2.0, 4.0));

        let mut d1 = FreeVec3::new(0.0, 1.0, 2.0);
        d1 /= 2.0;
        assert_eq!(d1, FreeVec3::new(0.0, 0.5, 1.0));

        assert_eq!(
            FreeVec3::new(0.0, 1.0, 2.0) * 2.0,
            FreeVec3::new(0.0, 2.0, 4.0)
        );
        assert_eq!(
            FreeVec3::new(0.0, 1.0, 2.0) / 2.0,
            FreeVec3::new(0.0, 0.5, 1.0)
        );
    }
}
