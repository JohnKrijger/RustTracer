use super::vector3::{self, normalized};
use cgmath::{Point3, Vector3};

pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn normalize(&mut self) {
        vector3::normalize(&mut self.direction)
    }

    pub fn normalized(self) -> Self {
        Ray {
            direction: normalized(self.direction),
            ..self
        }
    }
}
