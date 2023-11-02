use crate::color::Color;
use crate::math::{Point, Vector};
use crate::shapes::material::Material;

pub struct PrimaryHit {
    pos: Point,
    normal: Vector,
    distance: f32,
    material: Material,
    bvh_hit_count: i32,
}

impl PrimaryHit {
    pub fn new(
        pos: Point,
        normal: Vector,
        distance: f32,
        material: Material,
        bvh_hit_count: i32,
    ) -> PrimaryHit {
        PrimaryHit {
            pos,
            normal,
            distance,
            material,
            bvh_hit_count,
        }
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn albedo(&self) -> Color {
        self.material.color
    }
}
