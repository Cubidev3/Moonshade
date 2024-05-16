use std::collections::VecDeque;
use crate::math::ray::Ray;
use crate::surfaces::surface::SurfacePoint;

pub mod default_ray_shader;

pub trait RayShader {
    fn next_ray(&self, ray: Ray, surface_point: SurfacePoint, reflection_stack: &VecDeque<SurfacePoint>) -> Option<Ray>;
    fn on_intersection(&self, ray: Ray, surface_point: SurfacePoint, reflection_stack: &mut VecDeque<SurfacePoint>);
    fn reflection_count_hint(&self) -> usize;
}
