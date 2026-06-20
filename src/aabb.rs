use std::ops::Index;

use glam::DVec3;

use crate::{hit::HitRange, ray::Ray};

#[derive(Clone)]
pub struct AABB {
    pub x: HitRange,
    pub y: HitRange,
    pub z: HitRange,
}

pub const EMPTY: AABB = AABB {
    x: HitRange::new(0., 0.),
    y: HitRange::new(0., 0.),
    z: HitRange::new(0., 0.),
};

impl AABB {
    pub fn new(x: HitRange, y: HitRange, z: HitRange) -> Self {
        Self { x, y, z }
    }

    pub fn intersects(&self, r: &Ray, interval_t: &HitRange) -> Option<HitRange> {
        let mut interval_t = *interval_t;
        for axis in 0..3 {
            let ax = &self[axis];
            let adinv = 1.0 / r.dir[axis];
            let t0 = (ax.start - r.orig[axis]) * adinv;
            let t1 = (ax.end - r.orig[axis]) * adinv;
            if t0 < t1 {
                interval_t.start = interval_t.start.max(t0);
                interval_t.end = interval_t.end.min(t1);
            } else {
                interval_t.start = interval_t.start.max(t1);
                interval_t.end = interval_t.end.min(t0);
            }

            if interval_t.size() < 0. {
                return None;
            }
        }
        Some(interval_t)
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else if self.y.size() > self.z.size() {
            1
        } else {
            2
        }
    }
}

impl Index<usize> for AABB {
    type Output = HitRange;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of bounds"),
        }
    }
}

impl From<(DVec3, DVec3)> for AABB {
    fn from((a, b): (DVec3, DVec3)) -> Self {
        let x = HitRange::new_to_sorted(a.x, b.x);
        let y = HitRange::new_to_sorted(a.y, b.y);
        let z = HitRange::new_to_sorted(a.z, b.z);
        Self { x, y, z }
    }
}

impl From<(AABB, AABB)> for AABB {
    fn from((a, b): (AABB, AABB)) -> Self {
        let x = (a.x, b.x).into();
        let y = (a.y, b.y).into();
        let z = (a.z, b.z).into();
        Self { x, y, z }
    }
}

impl From<(&AABB, &AABB)> for AABB {
    fn from((a, b): (&AABB, &AABB)) -> Self {
        let x = (a.x, b.x).into();
        let y = (a.y, b.y).into();
        let z = (a.z, b.z).into();
        Self { x, y, z }
    }
}
