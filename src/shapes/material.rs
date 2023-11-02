use crate::color::Color;
#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub material_type: MaterialType,
    pub color: Color,
    pub emissive_color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MaterialType {
    Reflective,
    Refractive { refraction_index: f32 },
    Diffuse,
    Glossy { specualrity: f32 },
}
