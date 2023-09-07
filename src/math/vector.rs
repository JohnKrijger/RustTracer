use std::ops::{Div, DivAssign, Mul, MulAssign};

use cgmath::Vector3;

use super::Vector;

impl Vector {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::Scaled {
            base: Vector3::new(x, y, z),
        }
    }

    pub fn up() -> Self {
        Self::Normal {
            base: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        cgmath::dot(self.base(), other.base())
    }

    pub fn cross(self, other: Self) -> Self {
        Self::Scaled {
            base: cgmath::Vector3::cross(self.base(), other.base()),
        }
    }

    pub fn magnitude_squared(self) -> f32 {
        if let Vector::Normal { base } = self {
            1.0
        } else {
            self.dot(self)
        }
    }

    pub fn magnitude(&self) -> f32 {
        if let Vector::Normal { base } = self {
            1.0
        } else {
            f32::sqrt(self.magnitude_squared())
        }
    }

    pub fn normalize(&mut self) {
        if let Vector::Scaled { base } = *self {
            *self = self.normalized()
        }
    }

    pub fn normalized(self) -> Self {
        if let Vector::Scaled { base } = self {
            Vector::Normal {
                base: base / self.magnitude(),
            }
        } else {
            self
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Scaled {
            base: self.base() * rhs,
        }
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::Scaled {
            base: self * rhs.base(),
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Scaled {
            base: self.base() / rhs,
        }
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        *self = *self / rhs;
    }
}
