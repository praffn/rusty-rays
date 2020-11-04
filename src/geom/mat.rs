use crate::film::Color;
use crate::geom::HitInfo;
use crate::geom::Scene;
use crate::linalg::Ray;

pub trait Material {
    fn shade(&self, ray: &Ray, hit_info: &HitInfo, scene: &Scene) -> Color;
}

pub struct DiffuseMaterial {
    diffuse_reflection: f32,
    diffuse_color: Color,
    ambient_reflection: f32,
    ambient_color: Color,
}

impl DiffuseMaterial {
    pub fn new(
        diffuse_reflection: f32,
        diffuse_color: Color,
        ambient_reflection: f32,
        ambient_color: Color,
    ) -> Self {
        Self {
            diffuse_reflection,
            diffuse_color,
            ambient_reflection,
            ambient_color,
        }
    }
}

impl Material for DiffuseMaterial {
    fn shade(&self, ray: &Ray, hit_info: &HitInfo, scene: &Scene) -> Color {
        let inv_pi = 1.0 / std::f32::consts::PI;
        let normal = if ray.direction.dot(hit_info.normal) > 0.0 {
            -hit_info.normal
        } else {
            hit_info.normal
        };
        let normal_color = Color::new(normal.x, normal.y, normal.z);
        let mut base_color =
            self.ambient_color * self.ambient_reflection * scene.ambient_light.color();
        for light in &scene.lights {
            let light_direction = light.direction_from_point(hit_info.hit_point);
            let dp = normal.dot(light_direction);
            let nudged_hit_point = hit_info.hit_point.displace(normal * 1.0e-6);
            if dp > 0.0 && !light.illuminates_point(nudged_hit_point, &*scene.shape) {
                let a = self.diffuse_color * self.diffuse_reflection * inv_pi;
                let b = light.color() * (light.geometric_factor() / light.probability_density());
                let c = normal_color
                    * Color::new(light_direction.x, light_direction.y, light_direction.z);
                base_color += a * b * c
            } else {
                base_color += Color::black();
            }
        }
        base_color
    }
}

pub struct DebugMaterial {}

impl DebugMaterial {
    pub fn new() -> DebugMaterial {
        DebugMaterial {}
    }
}

impl Material for DebugMaterial {
    fn shade(&self, _: &Ray, _: &HitInfo, _: &Scene) -> Color {
        Color {
            r: 1.0,
            g: 1e-6,
            b: 1.0,
        }
    }
}
