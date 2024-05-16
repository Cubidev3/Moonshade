use crate::renderer::lens_shader::LensShader;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::vector::Vector;

pub struct PlanePerspectiveLensShader {
    pub top_left_point: Point,
    pub size: Vector
}

impl PlanePerspectiveLensShader {
    pub fn new(focal_length: f64, size: Vector) -> PlanePerspectiveLensShader {
        PlanePerspectiveLensShader { size, top_left_point: Point::new(0.0, 0.0, focal_length) - size / 2.0 }
    }
}

impl LensShader for PlanePerspectiveLensShader {
    fn ray_to_lens_point(&self, x: f64, y: f64) -> Option<Ray> {
        Some(Ray::from_points(
            Point::ZERO,
            self.top_left_point + Vector::component_wise_multiplication(self.size, Vector::new(x, y, 1.0))
        ))
    }
}