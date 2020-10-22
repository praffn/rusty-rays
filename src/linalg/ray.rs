use super::{Point3, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn point_at_distance(&self, distance: f32) -> Point3 {
        self.origin + self.direction * distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn rays_can_return_points() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let p = ray.point_at_distance(5.5);
        assert_approx_eq!(p.x, 0.0);
        assert_approx_eq!(p.y, 5.5);
        assert_approx_eq!(p.z, 0.0);
    }
}
