use crate::ray::Ray;
use crate::spaces::{FreeVec3, Point, UnitVec3, Vec3};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: FreeVec3,
    vertical: FreeVec3,
    u: FreeVec3,
    v: FreeVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
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

        let w = UnitVec3::from(lookfrom - Point::new(0., 0., 0.) - lookat);
        let u = UnitVec3::from(vup.cross(&w));
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
            u: u.into(),
            v,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = FreeVec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset)
                .into(),
        )
    }
}
