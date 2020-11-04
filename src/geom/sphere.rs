use super::{HitInfo, Shape};
use crate::geom::{mat, Material};
use crate::linalg::{smallest_greater_than_zero, solve_quadratic, Point3, Ray, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material: material,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        match solve_quadratic(a, b, c) {
            None => None,
            Some((t0, t1)) => match smallest_greater_than_zero(t0, t1) {
                None => None,
                Some(t) => {
                    let hit_point = ray.point_at_distance(t);
                    let normal = hit_point.to_vec3() * (1.0 / self.radius);
                    Some(HitInfo {
                        distance: t,
                        normal,
                        hit_point,
                        material: &self.material,
                    })
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn a_ray_not_intersecting_a_sphere_will_return_none() {
        let sphere = Sphere {
            center: Point3::new(3.0, 2.0, 0.0),
            radius: 1.0,
            material: Box::new(mat::DebugMaterial::new()),
        };
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let hit = sphere.hit(&ray);
        assert_eq!(hit.is_none(), true);
    }

    #[test]
    fn a_ray_intersecting_a_sphere_will_return_some_hitinfo_with_distance() {
        let sphere = Sphere {
            center: Point3::new(0.0, 2.0, 0.0),
            radius: 1.0,
            material: Box::new(mat::DebugMaterial::new()),
        };
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let hit = sphere.hit(&ray);
        assert_eq!(hit.is_some(), true);
        let hit_info = hit.unwrap();
        assert_approx_eq!(hit_info.distance, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_sphere_from_behind_will_return_none() {
        let sphere = Sphere {
            center: Point3::new(0.0, 2.0, 0.0),
            radius: 1.0,
            material: Box::new(mat::DebugMaterial::new()),
        };
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        let hit = sphere.hit(&ray);
        assert_eq!(hit.is_none(), true);
    }
}
