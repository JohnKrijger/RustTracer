#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Material {
    Reflective,
    Refractive { refraction_index: f32 },
    Diffuse,
    Glossy { specualrity: f32 },
}
