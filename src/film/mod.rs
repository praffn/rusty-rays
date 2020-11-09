use crate::linalg::Ray;

mod color;

pub trait Camera {
    fn get_rays_for_coordinate(&self, x: u32, y: u32) -> Vec<Ray>;
}

mod pinhole_camera;

pub use color::Color;
pub use pinhole_camera::{PinholeCamera, PinholeCameraConfig};
