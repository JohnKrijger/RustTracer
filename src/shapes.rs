use crate::math::{Point, Ray};
use crate::ray_hits::primary_hit::PrimaryHit;

use self::material::Material;

pub mod material;
pub mod plane;
pub mod sphere;

pub trait Shape {
    fn primary_intersection(&self, ray: Ray) -> Vec<PrimaryHit>;
    fn has_intersection_between(&self, from: Point, to: Point) -> bool;
    fn material(&self) -> Material;
}
