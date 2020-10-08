use crate::ray::Ray;
use crate::vec3::{BoundVec3, FreeVec3, UnitVec3};

pub struct Camera {
    origin: BoundVec3,
    lower_left_corner: BoundVec3,
    horizontal: FreeVec3,
    vertical: FreeVec3,
}

impl Camera {
    pub fn new(
        lookfrom: BoundVec3,
        lookat: FreeVec3,
        vup: FreeVec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = FreeVec3::from(UnitVec3::from(
            lookfrom - BoundVec3::new(0., 0., 0.) - lookat,
        ));
        let u = FreeVec3::from(UnitVec3::from(vup.cross(&w)));
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        Camera {
            lower_left_corner: origin - w - horizontal / 2.0 - vertical / 2.0,
            origin,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
                .into(),
        )
    }
}
