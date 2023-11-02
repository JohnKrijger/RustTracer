use std::{rc::Rc, slice::Iter};

use crate::{
    camera::Camera,
    shapes::{plane::Plane, sphere::Sphere, Shape},
};

pub struct Scene {
    camera: Camera,
    planes: Vec<Plane>,
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new(
        camera: Camera,
        planes: impl Iterator<Item = Plane>,
        spheres: impl Iterator<Item = Sphere>,
    ) -> Self {
        Self {
            camera,
            planes: planes.collect(),
            spheres: spheres.collect(),
        }
    }

    pub fn camera(self: &Self) -> &Camera {
        &self.camera
    }

    pub fn shapes(self: &Self) -> &dyn Iterator<Item = dyn Shape> {
        let s: dyn Iterator<Item = dyn Shape> = self.spheres.iter();
        self.spheres.iter().chain(self.planes.iter()).collect()
    }

    pub fn emissive_shapes(&self) -> dyn Iterator<Item = *const dyn Shape> {
        let emissive_shapes = self
            .shapes
            .iter()
            .filter(|shape| shape.material().emissive_color != None);
        emissive_shapes
    }
}
