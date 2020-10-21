use super::{Point3, Vec3};

pub struct OrthonormalBase {
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl OrthonormalBase {
    pub fn construct(direction: Vec3, up: Vec3) -> Self {
        let w = direction.normalize();
        let v = (up.cross(w)).normalize();
        let u = w.cross(v);

        Self { u, v, w }
    }

    pub fn apply_vector(self, v: Vec3) -> Vec3 {
        self.u * v.x + self.v * v.y + self.w * v.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn an_orthonormal_frame_can_be_applied_to_a_vector() {
        let direction = Vec3::new(0.0, 2.0, 1.0).normalize();
        let up = Vec3::new(1.0, 0.0, 1.0).normalize();
        let ob = OrthonormalBase::construct(direction, up);
        let v = Vec3::new(1.0, 2.0, -3.0);
        let r = ob.apply_vector(v);

        assert_approx_eq!(r.x, -0.5879773);
        assert_approx_eq!(r.y, -3.648091);
        assert_approx_eq!(r.z, 0.5879773);
    }
}
