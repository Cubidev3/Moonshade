use std::collections::VecDeque;
use crate::renderer::pixel_shader::PixelShader;
use crate::math::color::Color;
use crate::surfaces::surface::{Surface, SurfacePoint};

pub struct DefaultPixelShader;

impl PixelShader for DefaultPixelShader {
    fn final_color<S: Surface>(&self, stack: &VecDeque<SurfacePoint>, surface: &S) -> Color {
        if stack.is_empty() {
            return Color::BLACK;
        }

        stack.iter()
            .rfold(Color::WHITE, |accumulator, surface_point| surface_point.material.diffuse * accumulator)
    }
}