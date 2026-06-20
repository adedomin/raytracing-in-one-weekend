use std::{
    f64,
    ops::{Range, RangeFrom, RangeTo},
};

use glam::DVec3;

use crate::{aabb::AABB, ray::Ray};

pub mod lim {
    use crate::hit::HitRange;

    pub const ZERO_TO: HitRange = HitRange::new(0f64, f64::INFINITY);
}

pub struct HitRec {
    pub t: f64,
    pub p: DVec3,
    pub norm: DVec3,
    pub face: bool,
    pub ancillary: usize,
}

#[derive(Clone, Copy)]
pub struct HitRange {
    pub start: f64,
    pub end: f64,
}

impl Default for HitRange {
    fn default() -> Self {
        Self::new(f64::INFINITY, f64::NEG_INFINITY)
    }
}

impl From<Range<f64>> for HitRange {
    fn from(value: Range<f64>) -> Self {
        HitRange::new(value.start, value.end)
    }
}

impl From<RangeTo<f64>> for HitRange {
    fn from(value: RangeTo<f64>) -> Self {
        HitRange::new(0., value.end)
    }
}
impl From<RangeFrom<f64>> for HitRange {
    fn from(value: RangeFrom<f64>) -> Self {
        HitRange::new(value.start, f64::INFINITY)
    }
}

impl HitRange {
    pub const fn new(start: f64, end: f64) -> Self {
        assert!(!start.is_nan(), "start cannot be NaN");
        assert!(!end.is_nan(), "end cannot be NaN");
        Self { start, end }
    }

    pub const fn new_to_sorted(a: f64, b: f64) -> Self {
        assert!(!a.is_nan(), "start cannot be NaN");
        assert!(!b.is_nan(), "end cannot be NaN");
        Self {
            start: a.min(b),
            end: a.max(b),
        }
    }

    pub fn size(&self) -> f64 {
        self.end - self.start
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }

    pub fn contains(&self, x: f64) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn clamp(&self, x: f64) -> f64 {
        x.clamp(self.start, self.end)
    }

    pub fn pad(self, delta: f64) -> Self {
        let delta = delta.abs();
        HitRange {
            start: self.start - delta,
            end: self.end + delta,
        }
    }
}

impl From<(HitRange, HitRange)> for HitRange {
    fn from((a, b): (HitRange, HitRange)) -> Self {
        let start = a.start.min(b.start);
        let end = a.end.max(b.end);
        Self { start, end }
    }
}

impl HitRec {
    pub fn new_gen_face(r: &Ray, t: f64, p: DVec3, norm: DVec3) -> Self {
        let (face, norm) = if r.dir.dot(norm) < 0. {
            (true, norm)
        } else {
            (false, -norm)
        };
        Self {
            t,
            p,
            norm,
            face,
            ancillary: usize::MAX,
        }
    }

    pub fn set_ancillary(mut self, ancillary: usize) -> Self {
        self.ancillary = ancillary;
        self
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_lim: &HitRange) -> Option<HitRec>;
    fn bounding_box(&self) -> AABB;
}
