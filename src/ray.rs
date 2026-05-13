use glam::DVec3;

pub struct Ray {
    pub orig: DVec3,
    pub dir: DVec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.orig + (self.dir * t)
    }
}
