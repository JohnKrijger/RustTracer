use crate::{
    math::{Point, Vector},
    shapes::material::Material,
};

pub struct ShadowHit {
    pub pos: Point,
    pub normal: Vector,
    pub distance: f32,
    pub material: Material,
}

impl ShadowHit {
    pub fn new(pos: Point, normal: Vector, distance: f32, material: Material) -> Self {
        Self {
            pos,
            normal,
            distance,
            material,
        }
    }
}
