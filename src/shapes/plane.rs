use crate::math::vector3;
use cgmath::{Point3, Vector3};

pub struct Plane {
    normal: Vector3<f32>,
    d: f32,
}

impl Plane {
    pub fn from_point_and_normal(p: Point3<f32>, n: Vector3<f32>) -> Self {
        let normal = vector3::normalized(n);
        let d = cgmath::dot(normal, cgmath::vec3(p.x, p.y, p.z));
        Plane {
            normal: normal,
            d: d,
        }
    }
}
