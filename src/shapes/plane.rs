use crate::{
    color::{self, Color},
    math::{Point, Vector},
};

use super::{
    material::{self, Material},
    Shape,
};

pub struct Plane {
    normal: Vector,
    d: f32,
    color: Color,
    material: Material,
}

impl Plane {
    pub fn new(normal: Vector, d: f32, color: Color, material: Material) -> Self {
        Self {
            normal: normal.normalized(),
            d,
            color,
            material,
        }
    }

    pub fn from_point_and_normal(
        normal: Vector,
        point: Point,
        color: Color,
        material: Material,
    ) -> Self {
        let normal = normal.normalized();
        let d = normal.dot(point - Point::origin());
        Plane {
            normal,
            d,
            color,
            material,
        }
    }
}

impl Shape for Plane {
    fn primary_intersection(
        &self,
        ray: crate::math::Ray,
    ) -> Vec<crate::ray_hits::primary_hit::PrimaryHit> {
        todo!()
    }

    fn has_intersection_between(&self, a: Point, b: Point) -> bool {
        let a = self.normal.dot(a - Point::origin());
        let b = self.normal.dot(b - Point::origin());
        let d = self.d;
        a != d && b != d && ((a > d) != (b > d))
    }
}
