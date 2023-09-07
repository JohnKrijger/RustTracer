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
