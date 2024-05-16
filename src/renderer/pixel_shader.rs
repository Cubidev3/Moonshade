use std::collections::VecDeque;
use crate::math::color::Color;
use crate::surfaces::surface::{Surface, SurfacePoint};

pub mod default_pixel_shader;

pub trait PixelShader {
    fn final_color<S: Surface>(&self, stack: &VecDeque<SurfacePoint>, surface: &S) -> Color;
}
