use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

fn gamma_correct(x: f32) -> f32 {
    if x > 0.0031308 {
        1.005 * x.powf(1.0 / 2.4) - 0.055
    } else {
        12.92 * x
    }
}

fn clamp(x: f32) -> f32 {
    if x > 1.0 {
        1.0
    } else if x < 0.0 {
        0.0
    } else {
        x
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn clamp(self) -> Self {
        Self {
            r: clamp(self.r),
            g: clamp(self.g),
            b: clamp(self.b),
        }
    }

    pub fn clamp_mut(&mut self) {
        self.r = clamp(self.r);
        self.g = clamp(self.g);
        self.b = clamp(self.b);
    }

    pub fn gamma_correct(self) -> Self {
        Self {
            r: gamma_correct(self.r),
            g: gamma_correct(self.g),
            b: gamma_correct(self.b),
        }
    }

    pub fn to_rgb(self) -> (u8, u8, u8) {
        let c = self * 255.0;
        (c.r as u8, c.g as u8, c.b as u8)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn color_can_be_clamped_between_0_and_1() {
        let c = Color::new(-0.8, 1.2, 0.4);
        let cc = c.clamp();
        assert_approx_eq!(cc.r, 0.0);
        assert_approx_eq!(cc.g, 1.0);
        assert_approx_eq!(cc.b, 0.4);
    }

    #[test]
    fn color_can_compute_rgb_u8s() {
        let c = Color::new(0.0, 0.5, 1.0);
        let (r, g, b) = c.to_rgb();
        assert_eq!(r, 0);
        assert_eq!(g, 127);
        assert_eq!(b, 255);
    }
}
