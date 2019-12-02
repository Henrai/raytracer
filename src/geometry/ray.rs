use super::vector::Vec3D;

pub struct Ray {
    origin : Vec3D,
    direction : Vec3D,
}

impl Ray {
    pub fn getPoint(&self, t: f64) -> Vec3D {
        self.origin + self.direction*t
    }
}

