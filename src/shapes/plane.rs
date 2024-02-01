use crate::{
    math::{random, Point, Vector},
    ray_hits::primary_hit::PrimaryHit,
};

use super::{material::Material, Shape};

#[derive(Debug, Clone, Copy)]
pub struct Plane {
    normal: Vector,
    d: f32,
    material: Material,
}

impl Plane {
    pub fn new(normal: Vector, d: f32, material: Material) -> Self {
        Self {
            normal: normal.normalized(),
            d,
            material,
        }
    }

    pub fn from_point_and_normal(normal: Vector, point: Point, material: Material) -> Self {
        let normal = normal.normalized();
        let d = normal.dot(point - Point::origin());
        Plane {
            normal,
            d,
            material,
        }
    }
}

impl Shape for Plane {
    fn primary_intersection(
        &self,
        ray: crate::math::Ray,
    ) -> Vec<crate::ray_hits::primary_hit::PrimaryHit> {
        let ray = ray.normalized();
        let d = (self.d - (ray.origin() - Point::origin()).dot(self.normal))
            / ray.direction().dot(self.normal);

        if d <= 0.0 {
            return Vec::new();
        }

        let pos = ray.origin() + d * ray.direction();
        let hit = PrimaryHit::new(pos, self.normal, d, self.material, 0);
        return vec![hit];
    }

    fn has_intersection_between(&self, a: Point, b: Point) -> bool {
        let a = self.normal.dot(a - Point::origin());
        let b = self.normal.dot(b - Point::origin());
        let d = self.d;
        a != d && b != d && ((a > d) != (b > d))
    }

    fn material(&self) -> Material {
        self.material
    }

    fn random_point_on_front(&self, from: Point) -> (Point, Vector) {
        let dir = random::random_point_on_sphere() - Point::origin();
        let point =
            from + ((self.d + (from - Point::origin()).dot(dir)) / dir.dot(self.normal)) * dir;
        let normal = if (self.d + (from.from_origin().dot(self.normal)) > 0.0) {
            self.normal
        } else {
            -self.normal
        };
        (point, normal)
    }
}
