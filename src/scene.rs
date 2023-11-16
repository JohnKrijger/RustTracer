use std::rc::{Rc, Weak};

use rand::Rng;

use crate::{
    camera::Camera, color::Color, math::Ray, ray_hits::primary_hit::PrimaryHit, shapes::Shape,
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

        self.traverse_scene(ray)
            .map_or(Color::from_grayscale(0.2), |h| h.albedo())
    }

    fn traverse_scene(&self, ray: Ray) -> Option<PrimaryHit> {
        self.shapes
            .iter()
            .flat_map(|shape| (**shape).primary_intersection(ray))
            .filter(|h| h.distance() > 0.0)
            .min_by(|a, b| f32::partial_cmp(&a.distance(), &b.distance()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        camera::Camera,
        color::Color,
        math::{Point, Vector},
        shapes::{
            material::{Material, MaterialType},
            plane::Plane,
            sphere::Sphere,
            Shape,
        },
    };

    use super::Scene;

    fn camera() -> Camera {
        Camera::new(
            Point::new(0.0, 0.0, -10.0),
            Vector::new(0.0, 0.0, 1.0),
            120.0,
            8.0,
            10.0,
        )
    }
    fn sphere_1() -> Rc<dyn Shape> {
        Rc::new(Sphere::new(
            Point::new(0.0, 0.0, 0.0),
            2.0,
            Material::new(Color::new(0.2, 0.8, 0.3), MaterialType::Diffuse, None),
        ))
    }
    fn sphere_2() -> Rc<dyn Shape> {
        Rc::new(Sphere::new(
            Point::new(-5.0, -5.0, 5.0),
            3.0,
            Material::new(Color::from_grayscale(0.4), MaterialType::Diffuse, None),
        ))
    }
    fn plane_1() -> Rc<dyn Shape> {
        Rc::new(Plane::new(
            Vector::up(),
            -10.0,
            Material::new(Color::new(0.1, 0.1, 0.9), MaterialType::Diffuse, None),
        ))
    }

    #[test]
    fn test_scene_size() {
        let scene: Scene = Scene::new(camera(), vec![sphere_1(), sphere_2(), plane_1()]);
        assert_eq!(scene.shapes.len(), 3)
    }

    #[test]
    fn fail_test() {
        assert!(false);
    }
}
