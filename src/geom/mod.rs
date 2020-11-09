use crate::geom::mat::Material;
use crate::linalg::{Point3, Ray, Vec3};

pub mod mat;
mod scene;

pub use scene::Scene;

pub struct HitInfo<'a> {
    pub distance: f32,
    pub normal: Vec3,
    pub hit_point: Point3,
    pub material: &'a Box<dyn Material>,
}

pub trait Shape {
    fn hit(&self, ray: &Ray) -> Option<HitInfo>;
}

impl Shape for Vec<Box<dyn Shape>> {
    fn hit(&self, ray: &Ray) -> Option<HitInfo> {
        let mut hit: Option<HitInfo> = None;
        for shape in self {
            match shape.hit(ray) {
                None => {}
                Some(hit_a) => match &hit {
                    None => hit = Some(hit_a),
                    Some(hit_b) => {
                        if hit_a.distance < hit_b.distance {
                            hit = Some(hit_a)
                        }
                    }
                },
            }
        }
        hit
    }
}

mod sphere;

pub use sphere::Sphere;
