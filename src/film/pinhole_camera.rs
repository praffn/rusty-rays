use crate::linalg::{Point3, Vec3};

pub struct PinholeCamera {
    position: Point3,
    look_at: Point3,
    up: Vec3,
    zoom: f32,
    width: f32,
    height: f32,
    resolution_x: i32,
    resolution_y: i32,
}


