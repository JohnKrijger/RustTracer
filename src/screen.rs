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
                let screen_pos = Self::pixel_to_screen_space(pos, self.width, self.height);

                (screen_pos.0, screen_pos.1, color)
            })
            .collect()
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    fn pixel_to_screen_space(pos: usize, screen_width: usize, screen_height: usize) -> (f32, f32) {
        let screen_x = ((pos % screen_width) as f32 / (screen_width - 1) as f32) * 2.0 - 1.0;
        let screen_y = (((pos / screen_width) as f32 / (screen_height - 1) as f32) * -2.0 + 1.0)
            * screen_height as f32
            / screen_width as f32;
        (screen_x, screen_y)
    }
}

#[cfg(test)]
mod tests {
    use super::Screen;

    const WIDTH: usize = 300;
    const HEIGHT: usize = 200;

    fn screen() -> Screen {
        Screen::new(WIDTH, HEIGHT, "test_screen")
    }

    #[test]
    fn test_screen_size() {
        let screen = screen();
        assert_eq!(screen.width, WIDTH);
        assert_eq!(screen.height, HEIGHT);
        assert_eq!(screen.buffer.len(), WIDTH * HEIGHT);
    }

    #[test]
    fn test_pixel_to_screen_coordinates() {
        let aspect_ratio = HEIGHT as f32 / WIDTH as f32;
        let cases = [
            (0, -1.0, aspect_ratio),
            (WIDTH - 1, 1.0, aspect_ratio),
            (WIDTH * (HEIGHT - 1), -1.0, -aspect_ratio),
            (WIDTH * HEIGHT - 1, 1.0, -aspect_ratio),
        ];
        for (idx, x, y) in cases {
            assert_eq!(Screen::pixel_to_screen_space(idx, WIDTH, HEIGHT), (x, y))
        }
    }
}
