use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Vec3 {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn dot<T: Vec3>(&self, other: &T) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    fn cross<T: Vec3>(&self, other: &T) -> FreeVec3 {
        FreeVec3::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    fn length_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FreeVec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 for FreeVec3 {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}

impl Neg for FreeVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FreeVec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: Vec3> Add<T> for FreeVec3 {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        FreeVec3::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl<T: Vec3> Sub<T> for FreeVec3 {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        FreeVec3::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl Mul<f64> for FreeVec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        FreeVec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div<f64> for FreeVec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        FreeVec3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Vec3> AddAssign<T> for FreeVec3 {
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.x();
        self.y += rhs.y();
        self.z += rhs.z();
    }
}

impl<T: Vec3> SubAssign<T> for FreeVec3 {
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs.x();
        self.y -= rhs.y();
        self.z -= rhs.z();
    }
}

impl MulAssign<f64> for FreeVec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f64> for FreeVec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl FreeVec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        FreeVec3 { x, y, z }
    }

    pub fn random_in_unit_disk() -> Self {
        let mut rng = rand::thread_rng();
        loop {
            let p = FreeVec3::new(rng.gen_range(-1., 1.), rng.gen_range(-1., 1.), 0.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }
}

impl From<UnitVec3> for FreeVec3 {
    fn from(v: UnitVec3) -> Self {
        FreeVec3::new(v.x, v.y, v.z)
    }
}

pub fn reflection<T: Vec3 + Into<FreeVec3>, S: Vec3 + Into<FreeVec3>>(v: T, n: S) -> FreeVec3 {
    let dot = v.dot(&n);
    v.into() - n.into() * dot * 2.
}

pub fn refraction<T: Vec3 + Into<FreeVec3>, S: Vec3 + Into<FreeVec3>>(
    uv: T,
    n: S,
    etai_over_etat: f64,
) -> FreeVec3 {
    let uv_free: FreeVec3 = uv.into();
    let n_free: FreeVec3 = n.into();
    let cos_theta = (-uv_free).dot(&n_free);
    let r_out_perp = (n_free * cos_theta + uv_free) * etai_over_etat;
    let r_out_parallel = n_free * -(1. - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

#[derive(Copy, Clone)]
pub struct UnitVec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 for UnitVec3 {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}

impl UnitVec3 {
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
        let unit = v / v.length();
        UnitVec3 {
            x: unit.x(),
            y: unit.y(),
            z: unit.z(),
        }
    }
}

impl Neg for UnitVec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        UnitVec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for UnitVec3 {
    type Output = FreeVec3;

    fn mul(self, rhs: f64) -> Self::Output {
        FreeVec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
