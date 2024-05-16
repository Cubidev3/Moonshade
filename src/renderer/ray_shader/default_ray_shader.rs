use std::collections::VecDeque;
use crate::renderer::ray_shader::RayShader;
use crate::math::ray::Ray;
use crate::math::vector::Vector;
use crate::surfaces::surface::SurfacePoint;

pub struct DefaultRayShader {
    max_reflection_count: usize
}

impl DefaultRayShader {
    pub fn new(max_reflection_count: usize) -> DefaultRayShader {
        DefaultRayShader { max_reflection_count }
    }
}

impl RayShader for DefaultRayShader {
    fn next_ray(&self, ray: Ray, surface_point: SurfacePoint, reflection_stack: &VecDeque<SurfacePoint>) -> Option<Ray> {
        if reflection_stack.len() >= self.max_reflection_count {
            return None;
        }

        Some(Ray::new(
            surface_point.point,
            Vector::reflection(ray.direction, surface_point.normal)
        ))
    }

    fn on_intersection(&self, ray: Ray, surface_point: SurfacePoint, reflection_stack: &mut VecDeque<SurfacePoint>) {
        reflection_stack.push_back(surface_point);
    }

    fn reflection_count_hint(&self) -> usize {
        self.max_reflection_count
    }
}