use std::rc::Rc;

use camera::Camera;
use color::Color;
use math::{Point, Vector};
use minifb::Key;
use rand::{
    rngs::{OsRng, StdRng},
    Rng,
};
use scene::Scene;
use screen::Screen;
use shapes::{
    material::{Material, MaterialType},
    sphere::Sphere,
    Shape,
};

mod camera;
mod color;
mod math;
mod ray_hits;
mod scene;
mod screen;
mod shapes;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut screen = Screen::new(WIDTH, HEIGHT, "Hello rust");
    let camera = Camera::new(
        Point::new(0.0, 0.0, -10.0),
        Vector::new(0.0, 0.0, 1.0),
        120.0,
        8.0,
        10.0,
    );
    let sphere_1: Rc<dyn Shape> = Rc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        2.0,
        Material::new(Color::new(0.2, 0.8, 0.3), MaterialType::Diffuse, None),
    ));
    let sphere_2: Rc<dyn Shape> = Rc::new(Sphere::new(
        Point::new(-5.0, -5.0, 5.0),
        3.0,
        Material::new(Color::from_grayscale(0.4), MaterialType::Diffuse, None),
    ));
    let shapes = vec![sphere_1, sphere_2];

    let mut rng = OsRng::default();
    let scene = Scene::new(camera, shapes);

    while screen.is_open() && !screen.is_key_down(Key::Escape) {
        for (x, y, px) in screen.iter_over_pixels() {
            *px = scene.trace(x, y, &mut rng).to_byte_format();
        }

        screen.update();
    }
}
