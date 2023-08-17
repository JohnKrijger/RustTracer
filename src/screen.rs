use minifb::{Key, Window, WindowOptions};

use crate::color::Color;

pub struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    window: Window,
}

impl Screen {
    pub fn new(width: usize, height: usize, name: &str) -> Self {
        let mut window =
            Window::new(name, width, height, WindowOptions::default()).unwrap_or_else(|e| {
                panic!("{}", e);
            });

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Screen {
            width,
            height,
            buffer: vec![0; width * height],
            window,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title)
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.buffer[x + y * self.width] = color.to_byte_format();
        true
    }

    pub fn iter_over_pixels(&mut self) -> Vec<(f32, f32, &mut u32)> {
        self.buffer
            .iter_mut()
            .enumerate()
            .map(|(pos, color)| {
                (
                    ((pos % self.width) as f32) / (self.width as f32),
                    ((pos / self.width) as f32) / (self.height as f32),
                    color,
                )
            })
            .collect()
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}
