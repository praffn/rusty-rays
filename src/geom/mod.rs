use crate::linalg::{Ray, Vec3};

pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    // pub material: ?,
}

pub trait Shape {
    fn hit(&self, ray: &Ray) -> Option<HitInfo>;
}

impl Shape for Vec<Sphere> {
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
