use crate::ray::Ray;
use crate::vec3::{BoundVec3, FreeVec3};

pub struct Camera {
    origin: BoundVec3,
    lower_left_corner: BoundVec3,
    horizontal: FreeVec3,
    vertical: FreeVec3,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let focal_length = 1.0;

        let origin = BoundVec3::new(0.0, 0.0, 0.0);
        let horizontal = FreeVec3::new(viewport_width, 0.0, 0.0);
        let vertical = FreeVec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - FreeVec3::new(0.0, 0.0, focal_length);
        Camera {
            origin,
            lower_left_corner,
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
