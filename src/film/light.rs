use crate::film::Color;
use crate::linalg::{Point3, Ray, Vec3};
use crate::Shape;

pub trait Light {
    fn color(&self) -> Color;
    fn direction_from_point(&self, p: Point3) -> Vec3;
    fn illuminates_point(&self, p: Point3, shape: &dyn Shape) -> bool;
    fn geometric_factor(&self) -> f32;
    fn probability_density(&self) -> f32;
}

pub struct PointLight {
    position: Point3,
    color: Color,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Point3, color: Color, intensity: f32) -> Self {
        let intensity = if intensity < 0.0 {
            0.0
        } else if intensity > 1.0 {
            1.0
        } else {
            intensity
        };
        Self {
            position,
            color,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn color(&self) -> Color {
        return self.color * self.intensity;
    }

    fn direction_from_point(&self, p: Point3) -> Vec3 {
        p.distance_to(self.position).normalize()
    }

    fn illuminates_point(&self, p: Point3, shape: &dyn Shape) -> bool {
        let distance_vector = p.distance_to(self.position);
        let distance = distance_vector.length();
        let ray = Ray::new(p, distance_vector.normalize());
        match shape.hit(&ray) {
            None => true,
            Some(hit) => hit.distance > distance,
        }
    }

    fn geometric_factor(&self) -> f32 {
        1.0
    }

    fn probability_density(&self) -> f32 {
        1.0
    }
}

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
