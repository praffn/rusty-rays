use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

fn gamma(x: f32) -> u8 {
    let xf = if x > 0.0031308 {
        1.005 * x.powf(1.0 / 2.4) - 0.055
    } else {
        12.92 * x
    };
    (xf * 255.0) as u8
}

fn inverse_gamma(x: u8) -> f32 {
    let xf = x as f32 / 255.0;
    if xf <= 0.04045 {
        xf / 12.92
    } else {
        ((xf + 0.055) / 1.055).powf(2.4)
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

    pub fn black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn red() -> Self {
        Self {
            r: 1.0,
            g: 1e-6,
            b: 1e-6,
        }
    }

    pub fn green() -> Self {
        Self {
            r: 1e-6,
            g: 1.0,
            b: 1e-6,
        }
    }

    pub fn blue() -> Self {
        Self {
            r: 1e-6,
            g: 1e-6,
            b: 1.0,
        }
    }

    pub fn normalize(self) -> Self {
        let squared_length = self.r * self.r + self.g + self.g + self.b * self.b;
        if squared_length == 0.0 {
            self
        } else {
            let length = squared_length.sqrt();
            self * (1.0 / length)
        }
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

    pub fn to_rgb(self) -> [u8; 3] {
        [gamma(self.r), gamma(self.g), gamma(self.b)]
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: inverse_gamma(r),
            g: inverse_gamma(g),
            b: inverse_gamma(b),
        }
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

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
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

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
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
        let [r, g, b] = c.to_rgb();
        assert_eq!(r, 0);
        assert_eq!(g, 127);
        assert_eq!(b, 255);
    }
}
