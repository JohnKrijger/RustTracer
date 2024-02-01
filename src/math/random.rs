use rand::Rng;

use super::{point, Point};

pub fn random_point_in_circle() -> (f32, f32) {
    let rng = &mut rand::thread_rng();
    loop {
        let x = 2.0 * rng.gen::<f32>() - 1.0;
        let y = 2.0 * rng.gen::<f32>() - 1.0;
        if x * x + y * y <= 1.0 {
            return (x, y);
        }
    }
}

pub fn random_point_on_circle() -> (f32, f32) {
    let (x, y) = random_point_in_circle();
    let inverse_magnitude = 1.0 / f32::sqrt(x * x + y + y);
    (x * inverse_magnitude, y * inverse_magnitude)
}

pub fn random_point_in_sphere() -> Point {
    let rng = &mut rand::thread_rng();
    loop {
        let x = 2.0 * rng.gen::<f32>() - 1.0;
        let y = 2.0 * rng.gen::<f32>() - 1.0;
        let z = 2.0 * rng.gen::<f32>() - 1.0;
        if x * x + y * y + z * z <= 1.0 {
            return Point::new(x, y, z);
        }
    }
}

pub fn random_point_on_sphere() -> Point {
    Point::origin() + (random_point_in_sphere() - Point::origin()).normalized()
}
