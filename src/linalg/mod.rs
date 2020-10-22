mod orthonormal_base;
mod point3;
mod ray;
mod vec3;

pub use orthonormal_base::OrthonormalBase;
pub use point3::Point3;
pub use ray::Ray;
pub use vec3::Vec3;

pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        None
    } else {
        let t0 = (-b + disc.sqrt()) / 2.0 * a;
        let t1 = (-b - disc.sqrt()) / 2.0 * a;
        Some((t0, t1))
    }
}

pub fn smallest_greater_than_zero(a: f32, b: f32) -> Option<f32> {
    if a < 0.0 && b < 0.0 {
        None
    } else if a < 0.0 {
        Some(b)
    } else if b < 0.0 {
        Some(a)
    } else if a < b {
        Some(a)
    } else {
        Some(b)
    }
}
