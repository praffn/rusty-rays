use crate::film::Camera;
use crate::linalg::{OrthonormalBase, Point3, Ray, Vec3};

pub struct PinholeCameraConfig {
    pub position: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub zoom: f32,
    pub width: f32,
    pub height: f32,
    pub resolution_x: u32,
    pub resolution_y: u32,
}

pub struct PinholeCameraBuilder {
    config: PinholeCameraConfig,
}

impl PinholeCameraBuilder {
    pub fn build(&self) -> PinholeCamera {
        PinholeCamera::new(&self.config)
    }

    pub fn resolution(&mut self, resolution_x: u32, resolution_y: u32) -> &mut Self {
        self.config.resolution_x = resolution_x;
        self.config.resolution_y = resolution_y;
        self
    }

    pub fn dimensions(&mut self, width: f32, height: f32) -> &mut Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    pub fn harmonize_dimensions(&mut self, width: f32) -> &mut Self {
        let aspect_ratio = self.config.resolution_y as f32 / self.config.resolution_x as f32;
        self.config.width = width;
        self.config.height = width * aspect_ratio;
        self
    }

    pub fn zoom(&mut self, zoom: f32) -> &mut Self {
        self.config.zoom = zoom;
        self
    }

    pub fn position(&mut self, position: Point3) -> &mut Self {
        self.config.position = position;
        self
    }

    pub fn look_at(&mut self, look_at: Point3) -> &mut Self {
        self.config.look_at = look_at;
        self
    }
}

pub struct PinholeCamera {
    base: OrthonormalBase,
    position: Point3,
    pixel_width: f32,
    pixel_height: f32,
    half_res_x: f32,
    half_res_y: f32,
    neg_zoom: f32,
}

impl PinholeCamera {
    pub fn new(config: &PinholeCameraConfig) -> Self {
        let pixel_width = config.width / config.resolution_x as f32;
        let pixel_height = config.height / config.resolution_y as f32;
        let direction = config.position - config.look_at;
        let base = OrthonormalBase::construct(direction, config.up);

        let half_res_x = config.resolution_x as f32 / 2.0;
        let half_res_y = config.resolution_y as f32 / 2.0;

        Self {
            base,
            pixel_width,
            pixel_height,
            half_res_x,
            half_res_y,
            neg_zoom: -config.zoom,
            position: config.position,
        }
    }

    pub fn builder() -> PinholeCameraBuilder {
        PinholeCameraBuilder {
            config: PinholeCameraConfig {
                position: Point3::new(0.0, 0.0, -2.0),
                look_at: Point3::new(0.0, 0.0, 0.0),
                up: Vec3::new(0.0, 1.0, 0.0),
                zoom: 1.0,
                width: 1.0,
                height: 1.0,
                resolution_x: 512,
                resolution_y: 512,
            },
        }
    }
}

impl Camera for PinholeCamera {
    fn get_rays_for_coordinate(&self, x: u32, y: u32) -> Vec<Ray> {
        let n = 4;
        let mut rays = vec![];
        for xx in 0..n {
            for yy in 0..n {
                let x_offset = (1.0 + xx as f32) / (n as f32 + 1.0);
                let y_offset = (1.0 + yy as f32) / (n as f32 + 1.0);
                let px = self.pixel_width * (x as f32 - self.half_res_x + x_offset);
                let py = self.pixel_height * (y as f32 - self.half_res_y + y_offset);
                let pz = self.neg_zoom;

                let direction = self.base.apply(px, py, pz);
                let ray = Ray::new(self.position, direction);
                rays.push(ray);
            }
        }
        rays
    }
}
