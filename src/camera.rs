use crate::math::{random, Point, Ray, Vector};
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

    pub fn generate_ray(&self, screen_x: f32, screen_y: f32) -> Ray {
        let origin = self.generate_ray_origin();
        let target = self.generate_ray_target(screen_x, screen_y);
        Ray::new(origin, target - origin).normalized()
    }

    fn generate_ray_origin(&self) -> Point {
        let aperture_pos = random::random_point_in_circle();
        let aperture_diameter = self.apperture_diameter();
        self.pos
            + self.right * aperture_pos.0 * aperture_diameter
            + self.up * aperture_pos.1 * aperture_diameter
    }

    fn generate_ray_target(&self, screen_x: f32, screen_y: f32) -> Point {
        self.pos
            + self.forward * self.focal_length
            + self.right * screen_x * self.focal_length * self.sensor_size
            + self.up * screen_y * self.focal_length * self.sensor_size
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::math::{Point, Vector};

    use super::Camera;

    #[test]
    fn test_fov() {
        let fovs = [30.0, 60.0, 90.0, 120.0];

        for fov in fovs {
            let camera = Camera::new(Point::origin(), Vector::forward(), fov, 10.0, 0.0);
            let left_target = camera.generate_ray_target(-1.0, 0.0);
            let right_target = camera.generate_ray_target(1.0, 0.0);
            let left_vector = (left_target - Point::origin()).normalized();
            let right_vector = (right_target - Point::origin()).normalized();
            let angle = f32::acos(left_vector.dot(right_vector)) * 180.0 / PI;
            assert_eq!(angle, fov);
        }
    }
}
