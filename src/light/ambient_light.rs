use super::Light;
use crate::film::Color;
use crate::linalg::{Point3, Vec3};
use crate::Shape;

pub struct AmbientLight {
    color: Color,
    intensity: f32,
}

impl AmbientLight {
    pub fn new(color: Color, intensity: f32) -> Self {
        let intensity = if intensity < 0.0 {
            0.0
        } else if intensity > 1.0 {
            1.0
        } else {
            intensity
        };
        Self { color, intensity }
    }
}

impl Light for AmbientLight {
    fn color(&self) -> Color {
        return self.color * self.intensity;
    }

    fn direction_from_point(&self, _: Point3) -> Vec3 {
        Vec3::zero()
    }

    fn illuminates_point(&self, _: Point3, _: &dyn Shape) -> bool {
        true
    }

    fn geometric_factor(&self) -> f32 {
        1.0
    }

    fn probability_density(&self) -> f32 {
        1.0
    }
}
