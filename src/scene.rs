use std::{
    rc::{Rc, Weak},
    slice::Iter,
};

use rand::Rng;

use crate::{
    camera::Camera,
    color::Color,
    ray_hits::primary_hit::PrimaryHit,
    shapes::{self, plane::Plane, sphere::Sphere, Shape},
};

pub struct Scene {
    camera: Camera,
    shapes: Vec<Rc<dyn Shape>>,
}

impl Scene {
    pub fn new(camera: Camera, shapes: Vec<Rc<dyn Shape>>) -> Self {
        Self { camera, shapes }
    }

    pub fn camera(self: &Self) -> &Camera {
        &self.camera
    }

    pub fn shapes(self: &mut Self) -> Vec<Rc<dyn Shape>> {
        self.shapes.iter().map(|s| Rc::clone(s)).collect()
    }

    pub fn emissive_shapes(&self) -> Vec<Weak<dyn Shape>> {
        self.shapes
            .iter()
            .filter(|shape: &&Rc<dyn Shape>| shape.material().emissive_color != None)
            .map(|s| Rc::downgrade(&(*s)))
            .collect()
    }

    pub fn trace(&self, screen_x: f32, screen_y: f32, rng: &mut impl Rng) -> Color {
        let ray = self.camera.generate_ray(screen_x, screen_y, rng);
        let hit: Option<PrimaryHit> = None;
        for shape in self.shapes.iter() {
            let hit = (**shape)
                .primary_intersection(ray)
                .iter()
                .filter(|h| h.distance() > f32::EPSILON)
                .min_by(|a, b| f32::partial_cmp(&a.distance(), &b.distance()).unwrap());
        }
        hit.map_or(Color::from_grayscale(0.0), |h| h.albedo())
    }
}
