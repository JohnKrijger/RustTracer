use super::{Point, Vector};

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            base: cgmath::Point3::new(x, y, z),
        }
    }

    pub fn origin() -> Self {
        Point::new(0.0, 0.0, 0.0)
    }

    pub fn slight_offset(self: Self, normal: Vector) -> Self {
        self + f32::EPSILON * normal
    }

    pub fn from_origin(self: Self) -> Vector {
        self - Self::origin()
    }
}
