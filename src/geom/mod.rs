use crate::linalg::{Ray, Vec3};

pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    // pub material: ?,
}

pub trait Shape {
    fn hit(&self, ray: &Ray) -> Option<HitInfo>;
}

mod sphere;

pub use sphere::Sphere;
