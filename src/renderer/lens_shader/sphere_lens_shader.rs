use std::f64::consts::{PI, TAU};
use crate::math::point::Point;
use crate::math::ray::Ray;
use crate::math::transformation::rotation::Rotation;
use crate::math::vector::Vector;
use crate::renderer::lens_shader::LensShader;

pub struct SphereLensShader {
    amplitude_x: f64,
    amplitude_y: f64,
    radius: f64
}

impl SphereLensShader {
    pub fn new(amplitude_x: f64, amplitude_y: f64, radius: f64) -> SphereLensShader {
        SphereLensShader { amplitude_x, amplitude_y, radius }
    }
    
    pub fn full_sphere(radius: f64) -> SphereLensShader {
        SphereLensShader { amplitude_x: TAU, amplitude_y: PI, radius }
    }
}

impl LensShader for SphereLensShader {
    fn ray_to_lens_point(&self, x: f64, y: f64) -> Option<Ray> {
        let direction = Vector::FORWARD * self.radius;
        let direction = Rotation::on_y(self.amplitude_x * (x - 0.5)) * direction;
        let direction = Rotation::on_x(self.amplitude_y * (y - 0.5)) * direction;
        
        Some(Ray::new(
            Point::ZERO,
            direction
        ))
    }
}