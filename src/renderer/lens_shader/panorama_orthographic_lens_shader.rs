use std::f64::consts::TAU;
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::rotation::Rotation;
use crate::math::transformation::translation::Translation;
use crate::math::vector::Vector;
use crate::renderer::lens_shader::LensShader;

pub struct PanoramaOrthographicLensShader {
    amplitude: f64,
    height: f64,
    radius: f64
}

impl PanoramaOrthographicLensShader {
    pub fn new(amplitude: f64, height: f64, radius: f64) -> PanoramaOrthographicLensShader {
        PanoramaOrthographicLensShader {
            amplitude, height, radius
        }
    }
    
    pub fn full_amplitude(height: f64, radius: f64) -> PanoramaOrthographicLensShader {
        PanoramaOrthographicLensShader {
            amplitude: TAU,
            height, radius
        }
    }
}

impl LensShader for PanoramaOrthographicLensShader{
    fn ray_to_lens_point(&self, x: f64, y: f64) -> Option<Ray> {
        let ray = Ray::new(Point::ZERO, Vector::FORWARD * self.radius);
        let ray = Rotation::on_y(self.amplitude * (x - 0.5)) * ray;
        let ray = Translation::new(Vector::new(0.0, self.height * (y - 0.5), 0.0)) * ray;

        Some(ray)
    }
}