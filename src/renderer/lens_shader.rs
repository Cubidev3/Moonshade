use crate::math::ray::Ray;

pub mod plane_orthographic_lens_shader;
pub mod plane_perspective_lens_shader;
pub mod sphere_lens_shader;
pub mod panorama_perspective_lens_shader;
pub mod panorama_orthographic_lens_shader;

pub trait LensShader {
    fn ray_to_lens_point(&self, x: f64, y: f64) -> Option<Ray>;
}
