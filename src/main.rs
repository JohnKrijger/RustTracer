use color::Color;
use minifb::Key;
use screen::Screen;
mod color;
mod screen;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT, "Hello rust");

    while screen.is_open() && !screen.is_key_down(Key::Escape) {
        if false {
            for (x, y, px) in screen.iter_over_pixels() {
                *px = Color::new(x, y, (x - 0.5) * (x - 0.5) + (y - 0.5) * (y - 0.5))
                    .to_byte_format();
            }
        } else {
            for x in 0..screen.width() {
                for y in 0..screen.height() {
                    screen.draw_pixel(x, y, {
                        let x = (x as f32) / screen.width() as f32;
                        let y = (y as f32) / screen.height() as f32;
                        Color::new(x, y, (x - 0.5) * (x - 0.5) + (y - 0.5) * (y - 0.5))
                    });
                }
            }
        }

        screen.update();
    }
}
