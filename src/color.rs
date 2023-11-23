use std::ops::{Add, Div, Mul};

use cgmath::{num_traits::MulAdd, Angle};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_grayscale(value: f32) -> Self {
        Self {
            red: value,
            green: value,
            blue: value,
        }
    }

    pub fn to_byte_format(&self) -> u32 {
        let red: u8 = if self.red > 1.0 {
            255
        } else {
            (self.red * 255.0) as u8
        };
        let green: u8 = if self.green > 1.0 {
            255
        } else {
            (self.green * 255.0) as u8
        };
        let blue: u8 = if self.blue > 1.0 {
            255
        } else {
            (self.blue * 255.0) as u8
        };
        let result: u32 = ((red as u32) << 16) | ((green as u32) << 8) | blue as u32;
        result
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}
