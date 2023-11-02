use crate::math::{random::random_point_in_circle, Point, Ray, Vector};
use rand::Rng;
use std::f32::consts::PI;

pub struct Camera {
    pos: Point,
    forward: Vector,
    up: Vector,
    right: Vector,
    sensor_size: f32,
    focal_length: f32,
    focal_ratio: f32,
}

impl Camera {
    pub fn new(
        pos: Point,
        forward: Vector,
        field_of_view: f32,
        focal_length: f32,
        focal_ratio: f32,
    ) -> Self {
        let forward = forward.normalized();
        let right = forward.cross(Vector::up()).normalized();
        let up = right.cross(forward).normalized();
        let sensor_size = 2.0 * f32::tan(PI * field_of_view / 360.0);

        Self {
            pos,
            forward,
            up,
            right,
            sensor_size,
            focal_length,
            focal_ratio,
        }
    }

    fn apperture_diameter(&self) -> f32 {
        self.focal_length / self.focal_ratio
    }

    fn generate_ray(self, screen_x: f32, screen_y: f32, rng: &mut impl Rng) -> Ray {
        let aperture_pos = random_point_in_circle(rng);
        let aperture_diameter = self.apperture_diameter();
        let origin = self.pos
            + self.right * aperture_pos.0 * aperture_diameter
            + self.up * aperture_pos.1 * aperture_diameter;
        let target = self.pos
            + self.forward * self.focal_length
            + self.right * screen_x * self.focal_ratio * self.focal_length
            + self.up * screen_y * self.focal_ratio * self.focal_length;
        Ray::new(origin, target - origin)
    }
}
