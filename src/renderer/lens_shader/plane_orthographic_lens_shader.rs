use crate::renderer::lens_shader::LensShader;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::vector::Vector;

pub struct PlaneOrthographicalLensShader {
    pub top_left_point: Point,
    pub size: Vector
}

impl PlaneOrthographicalLensShader {
    pub fn new(size: Vector) -> PlaneOrthographicalLensShader {
        PlaneOrthographicalLensShader { size, top_left_point: Point::ZERO - size / 2.0 }
    }
}

impl LensShader for PlaneOrthographicalLensShader {
    fn ray_to_lens_point(&self, x: f64, y: f64) -> Option<Ray> {
        Some(Ray::new(
            self.top_left_point + Vector::component_wise_multiplication(self.size, Vector::new(x, y, 1.0)),
            Vector::FORWARD
        ))
    }
}