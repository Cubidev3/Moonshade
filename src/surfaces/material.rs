use crate::math::color::Color;

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse: Color
}

impl Material {
    pub fn new(diffuse: Color) -> Material {
        Material { diffuse }
    }
}