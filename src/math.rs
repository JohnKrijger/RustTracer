pub mod point;
pub mod ray;
pub mod vector;

use std::ops::{Add, AddAssign, Neg, Sub, SubAssign};

use cgmath::{Point3, Vector3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    base: Point3<f32>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Vector {
    Normal { base: Vector3<f32> },
    Scaled { base: Vector3<f32> },
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Vector {
    fn base(self) -> Vector3<f32> {
        match self {
            Vector::Normal { base } => base,
            Vector::Scaled { base } => base,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            base: self.base + rhs.base(),
        }
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.base += rhs.base()
    }
}

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector::Scaled {
            base: self.base() + rhs.base(),
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Vector::Normal { base } => Vector::Normal { base: -base },
            Vector::Scaled { base } => Vector::Scaled { base: -base },
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::Scaled {
            base: self.base() - rhs.base(),
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            base: self.base - rhs.base(),
        }
    }
}

impl SubAssign<Vector> for Point {
    fn sub_assign(&mut self, rhs: Vector) {
        self.base -= rhs.base()
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::Scaled {
            base: self.base - rhs.base,
        }
    }
}
