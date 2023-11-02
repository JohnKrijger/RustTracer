use crate::color::Color;
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub material_type: MaterialType,
    pub emissive_color: Option<Color>,
}

impl Material {
    pub fn new(color: Color, material_type: MaterialType, emissive_color: Option<Color>) -> Self {
        Self {
            color,
            material_type,
            emissive_color,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialType {
    Reflective,
    Refractive { refraction_index: f32 },
    Diffuse,
    Glossy { specualrity: f32 },
}
