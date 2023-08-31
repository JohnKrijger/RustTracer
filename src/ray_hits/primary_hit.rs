use crate::color::Color;
use crate::math::{Point, Vector};
use crate::shapes::material::Material;

pub struct PrimaryHit {
    pos: Point,
    normal: Vector,
    albedo: Color,
    material: Material,
    bvh_hit_count: i32,
}

impl PrimaryHit {
    pub fn new(
        pos: Point,
        normal: Vector,
        albedo: Color,
        material: Material,
        bvh_hit_count: i32,
    ) -> PrimaryHit {
        PrimaryHit {
            pos: pos,
            normal: normal,
            albedo: albedo,
            material: material,
            bvh_hit_count: bvh_hit_count,
        }
    }
}
