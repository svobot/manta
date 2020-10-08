use crate::ray::Ray;
use crate::vec3::{BoundVec3, FreeVec3, UnitVec3};

pub struct Camera {
    origin: BoundVec3,
    lower_left_corner: BoundVec3,
    horizontal: FreeVec3,
    vertical: FreeVec3,
    u: FreeVec3,
    v: FreeVec3,
    w: FreeVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: BoundVec3,
        lookat: FreeVec3,
        vup: FreeVec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
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
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        Camera {
            lower_left_corner: origin - w * focus_dist - horizontal / 2.0 - vertical / 2.0,
            lens_radius: aperture / 2.,
            origin,
            horizontal,
            vertical,
            u,
            v,
            w,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = FreeVec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * *rd.x() + self.v * *rd.y();

        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset)
                .into(),
        )
    }
}
