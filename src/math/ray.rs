use super::{Point, Ray, Vector};

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vector {
        self.direction
    }
}
