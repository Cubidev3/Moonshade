use std::f64::consts::TAU;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::rotation::Rotation;
use crate::math::vector::Vector;
use crate::renderer::lens_shader::LensShader;

pub struct PanoramaPerspectiveLensShader {
    amplitude: f64,
    height: f64,
    radius: f64
}

impl PanoramaPerspectiveLensShader {
    pub fn new(amplitude: f64, height: f64, radius: f64) -> PanoramaPerspectiveLensShader {
        PanoramaPerspectiveLensShader {
            amplitude, height, radius
        }
    }
    
    pub fn full_amplitude(height: f64, radius: f64) -> PanoramaPerspectiveLensShader {
        PanoramaPerspectiveLensShader {
            amplitude: TAU,
            height, radius
        }
    }
}

impl LensShader for PanoramaPerspectiveLensShader {
    fn ray_to_lens_point(&self, x: f64, y: f64) -> Option<Ray> {
        let direction = Vector::FORWARD * self.radius;
        let direction = Rotation::on_y(self.amplitude * (x - 0.5)) * direction;
        let direction = direction + Vector::UP * self.height * (y - 0.5);
        
        Some(Ray::new(
            Point::ZERO,
            direction
        ))
    }
}