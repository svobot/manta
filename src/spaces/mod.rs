/// [TODO] Find a better name than `spaces` for this module
pub mod point;
pub mod vec3;

pub use point::Point;
pub use vec3::{FreeVec3, UnitVec3, Vec3};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_arithmetics() {
        assert_eq!(
            point::Point::new(3.0, 3.0, 3.0) + FreeVec3::new(0.0, 1.0, 2.0),
            Point::new(3.0, 4.0, 5.0)
        );
        assert_eq!(
            Point::new(3.0, 3.0, 3.0) - FreeVec3::new(0.0, 1.0, 2.0),
            Point::new(3.0, 2.0, 1.0)
        );
        assert_eq!(
            Point::new(3.0, 3.0, 3.0) - Point::new(0.0, 1.0, 2.0),
            FreeVec3::new(3.0, 2.0, 1.0)
        );
    }

    #[test]
    fn free_arithmmetics() {
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
