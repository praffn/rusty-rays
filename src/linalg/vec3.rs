use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let inverse = 1.0 / rhs;
        self * inverse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn vectors_have_a_length() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let r = v.length();
        assert_approx_eq!(r, 3.7416573868);
    }

    #[test]
    fn vectors_have_a_squared_length() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let r = v.length_squared();
        assert_approx_eq!(r, 14.0);
    }

    #[test]
    fn vectors_can_be_added() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let w = Vec3::new(0.5, 0.1, -0.8);
        let r = v + w;
        assert_approx_eq!(r.x, 1.5);
        assert_approx_eq!(r.y, 2.1);
        assert_approx_eq!(r.z, 2.2);
    }

    #[test]
    fn vectors_can_be_subtracted() {
        let v = Vec3::new(1.5, 2.9, -8.0);
        let w = Vec3::new(-5.0, 5.0, 5.5);
        let r = v - w;
        assert_approx_eq!(r.x, 6.5);
        assert_approx_eq!(r.y, -2.1);
        assert_approx_eq!(r.z, -13.5)
    }

    #[test]
    fn vectors_can_be_multiplied_by_a_scalar() {
        let v = Vec3::new(1.5, -2.0, -0.5);
        let r = v * 2.5;
        assert_approx_eq!(r.x, 3.75);
        assert_approx_eq!(r.y, -5.0);
        assert_approx_eq!(r.z, -1.25);
    }

    #[test]
    fn vectors_can_be_divided_by_a_scalar() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let r = v / 2.0;
        assert_approx_eq!(r.x, 0.5);
        assert_approx_eq!(r.y, -1.0);
        assert_approx_eq!(r.z, 1.5);
    }

    #[test]
    fn two_vectors_have_a_dot_product() {
        let v = Vec3::new(1.0, -2.0, 3.0);
        let w = Vec3::new(0.0, 5.0, 2.5);
        let r = v.dot(w);
        assert_approx_eq!(r, -2.5);
    }

    #[test]
    fn two_vectors_have_a_cross_product() {
        let v = Vec3::new(1.0, 0.0, 0.0);
        let w = Vec3::new(0.0, 1.0, 0.0);
        let r = v.cross(w);
        assert_approx_eq!(r.x, 0.0);
        assert_approx_eq!(r.y, 0.0);
        assert_approx_eq!(r.z, 1.0);
    }
}
