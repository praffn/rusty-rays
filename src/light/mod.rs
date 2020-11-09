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

mod ambient_light;
mod point_light;

pub use ambient_light::AmbientLight;
pub use point_light::PointLight;
