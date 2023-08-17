use crate::color::Color;
use crate::math::{ray, vector3};
use cgmath::{Point3, Vector3};

pub struct PrimaryHit {
    pos: Point3<f32>,
    normal: Vector3<f32>,
    albedo: Color,
    bvh_hit_count: i32,
}
