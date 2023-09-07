use crate::{
    color::Color,
    math::{Point, Ray},
    ray_hits::primary_hit::PrimaryHit,
};

use super::{
    material::{self, Material},
    Shape,
};

pub struct Sphere {
    center: Point,
    radius: f32,
    color: Color,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, color: Color, material: Material) -> Self {
        Self {
            center,
            radius,
            color,
            material,
        }
    }
}

impl Shape for Sphere {
    fn primary_intersection(&self, ray: Ray) -> Vec<PrimaryHit> {
        let to_centre = self.center - ray.origin();
        let dir = ray.direction();
        let cos = to_centre.dot(dir);
        let discriminant = cos.powi(2) - to_centre.magnitude_squared() + self.radius.powi(2);
        let distances = match discriminant {
            d if d > 0.0 => {
                let d_sqrt = d.sqrt();
                vec![cos - d_sqrt, cos + d_sqrt]
            }
            d if d == 0.0 => vec![cos],
            x => vec![],
        };
        let color = self.color;
        let material = self.material;
        distances
            .iter()
            .map(|d| ray.origin() + ray.direction() * *d)
            .map(|pos| PrimaryHit::new(pos, (pos - self.center).normalized(), color, material, 0))
            .collect::<Vec<PrimaryHit>>()
    }

    fn has_intersection_between(&self, from: Point, to: Point) -> bool {
        let to_centre = self.center - from;
        let dir = (to - from).normalized();
        to_centre.dot(dir).powi(2) - to_centre.magnitude_squared() >= self.radius.powi(2)
    }
}
