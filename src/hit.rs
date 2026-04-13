use std::ops::RangeInclusive;

use crate::{ray::Ray, vec3::Vec3};

pub mod lim {
    use std::ops::RangeInclusive;

    pub const ZERO_TO: RangeInclusive<f64> = 0f64..=f64::INFINITY;
}

pub struct HitRec {
    pub t: f64,
    pub p: Vec3,
    pub norm: Vec3,
    pub face: bool,
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
    fn hit(&self, r: &Ray, t_lim: &RangeInclusive<f64>) -> Option<HitRec>;
}
