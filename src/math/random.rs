use rand::Rng;

pub fn random_point_in_circle(rng: &mut impl Rng) -> (f32, f32) {
    loop {
        let x = 2.0 * rng.gen::<f32>() - 1.0;
        let y = 2.0 * rng.gen::<f32>() - 1.0;
        if x * x + y * y <= 1.0 {
            return (x, y);
        }
    }
}

pub fn random_point_on_circle(rng: &mut impl Rng) -> (f32, f32) {
    let (x, y) = random_point_in_circle(rng);
    let inverse_magnitude = 1.0 / f32::sqrt(x * x + y + y);
    (x * inverse_magnitude, y * inverse_magnitude)
}

pub fn random_point_in_sphere(rng: &mut impl Rng) -> (f32, f32, f32) {
    loop {
        let x = 2.0 * rng.gen::<f32>() - 1.0;
        let y = 2.0 * rng.gen::<f32>() - 1.0;
        let z = 2.0 * rng.gen::<f32>() - 1.0;
        if x * x + y * y + z * z <= 1.0 {
            return (x, y, z);
        }
    }
}

pub fn random_point_on_sphere(rng: &mut impl Rng) -> (f32, f32, f32) {
    let (x, y, z) = random_point_in_sphere(rng);
    let inverse_magnitude = 1.0 / f32::sqrt(x * x + y + y + z * z);
    (
        x * inverse_magnitude,
        y * inverse_magnitude,
        z * inverse_magnitude,
    )
}
