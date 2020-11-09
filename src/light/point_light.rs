use super::Light;
use crate::film::Color;
use crate::linalg::{Point3, Ray, Vec3};
use crate::Shape;

pub struct PointLight {
    position: Point3,
    color: Color,
    intensity: f32,
}

impl PointLight {
    pub fn new(position: Point3, color: Color, intensity: f32) -> Self {
        let intensity = if intensity < 0.0 {
            0.0
        } else if intensity > 1.0 {
            1.0
        } else {
            intensity
        };
        Self {
            position,
            color,
            intensity,
        }
    }
}

impl Light for PointLight {
    fn color(&self) -> Color {
        return self.color * self.intensity;
    }

    fn direction_from_point(&self, p: Point3) -> Vec3 {
        p.distance_to(self.position).normalize()
    }

    fn illuminates_point(&self, p: Point3, shape: &dyn Shape) -> bool {
        let distance_vector = p.distance_to(self.position);
        let distance = distance_vector.length();
        let ray = Ray::new(p, distance_vector.normalize());
        match shape.hit(&ray) {
            None => true,
            Some(hit) => hit.distance > distance,
        }
    }

    fn geometric_factor(&self) -> f32 {
        1.0
    }

    fn probability_density(&self) -> f32 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geom::{mat, Sphere};
    use assert_approx_eq::assert_approx_eq;

    fn generate_shapes() -> Vec<Box<dyn Shape>> {
        vec![Box::new(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            0.5,
            Box::new(mat::DebugMaterial::new()),
        ))]
    }

    #[test]
    fn pointlight_illuminates_point_with_no_obstruction() {
        let shapes = generate_shapes();
        let light = PointLight::new(Point3::new(0.0, 0.0, 0.0), Color::red(), 0.5);
        let illuminates = light.illuminates_point(Point3::new(0.0, -1.0, 0.0), &shapes);
        assert_eq!(illuminates, true);
    }

    #[test]
    fn pointlight_does_not_illuminate_point_with_obstruction() {
        let shapes = generate_shapes();
        let light = PointLight::new(Point3::new(0.0, 0.0, 0.0), Color::red(), 0.5);
        let illuminates = light.illuminates_point(Point3::new(0.0, 2.0, 0.0), &shapes);
        assert_eq!(illuminates, false);
    }
}
