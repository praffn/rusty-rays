use super::Vec3;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn distance_to(self, point: Point3) -> Vec3 {
        point - self
    }

    pub fn displace(self, displacement: Vec3) -> Self {
        self + displacement
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Point3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn a_distance_vector_can_be_computed() {
        let p = Point3::new(1.0, -2.0, 3.0);
        let q = Point3::new(-4.0, 5.0, -6.0);
        let d = p.distance_to(q);
        assert_approx_eq!(d.x, -5.0);
        assert_approx_eq!(d.y, 7.0);
        assert_approx_eq!(d.z, -9.0);
    }

    #[test]
    fn a_point_can_be_displaced_by_a_vector() {
        let p = Point3::new(1.5, -2.5, 3.5);
        let v = Vec3::new(0.0, -1.0, -5.0);
        let q = p.displace(v);
        assert_approx_eq!(q.x, 1.5);
        assert_approx_eq!(q.y, -3.5);
        assert_approx_eq!(q.z, -1.5);
    }
}
