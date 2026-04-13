use std::{
    f64,
    ops::{Range, RangeFrom, RangeTo},
};

use crate::{ray::Ray, vec3::Vec3};

pub mod lim {
    use crate::hit::HitRange;

    pub const ZERO_TO: HitRange = HitRange::new(0f64, f64::INFINITY);
}

pub struct HitRec {
    pub t: f64,
    pub p: Vec3,
    pub norm: Vec3,
    pub face: bool,
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

    pub fn size(&self) -> f64 {
        self.end - self.start
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.start < x && x < self.end
    }

    pub fn contains(&self, x: f64) -> bool {
        self.start <= x && x <= self.end
    }
}

impl HitRec {
    pub fn new_gen_face(r: &Ray, t: f64, p: Vec3, norm: Vec3) -> Self {
        let (face, norm) = if r.dir.dot(norm) < 0. {
            (true, norm)
        } else {
            (false, -norm)
        };
        Self { t, p, norm, face }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_lim: &HitRange) -> Option<HitRec>;
}
