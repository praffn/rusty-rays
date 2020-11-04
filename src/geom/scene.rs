use super::mat::Material;
use crate::film::light::Light;
use crate::film::Color;
use crate::linalg::Ray;
use crate::Shape;

pub struct Scene {
    pub lights: Vec<Box<dyn Light>>,
    pub ambient_light: Box<dyn Light>,
    pub shape: Box<dyn Shape>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Color {
        match self.shape.hit(ray) {
            None => Color::black(),
            Some(hit_info) => hit_info.material.shade(&ray, &hit_info, self),
        }
    }
}
