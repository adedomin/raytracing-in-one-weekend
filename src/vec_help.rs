use glam::DVec3;

use crate::{hit::HitRange, render::RGB};

pub fn rand_double() -> f64 {
    rand::random_range(0.0..1.)
}

pub fn rand_vec3_range(r: std::ops::Range<f64>) -> DVec3 {
    DVec3::new(
        rand::random_range(r.clone()),
        rand::random_range(r.clone()),
        rand::random_range(r),
    )
}

pub fn rand_vec3() -> DVec3 {
    rand_vec3_range(0.0..1.)
}

pub fn safe_random_unit_vec() -> DVec3 {
    loop {
        if let Some(norm) = rand_vec3_range(-1.0..1.).try_normalize() {
            return norm;
        };
    }
}

pub fn not_near_zero(norm: DVec3) -> Option<DVec3> {
    let len_recip = norm.length_recip();
    (len_recip.is_finite() && len_recip > 0.0).then_some(norm)
}

pub fn rand_on_hemi(norm: DVec3) -> DVec3 {
    let r = safe_random_unit_vec();
    if r.dot(norm) > 0.0 { r } else { -r }
}

const COLOR_INTERVAL: HitRange = HitRange::new(0., 0.999);

fn lin_to_srgb_gamma(lin: f64) -> f64 {
    if lin <= 0.0031308 {
        12.92 * lin
    } else {
        1.055 * lin.powf(1. / 2.4) - 0.055
    }
}

pub fn to_srgb(value: DVec3) -> RGB {
    let DVec3 { x: r, y: g, z: b } = value;
    let r = lin_to_srgb_gamma(r);
    let g = lin_to_srgb_gamma(g);
    let b = lin_to_srgb_gamma(b);

    let r = (COLOR_INTERVAL.clamp(r) * 256f64) as u8;
    let g = (COLOR_INTERVAL.clamp(g) * 256f64) as u8;
    let b = (COLOR_INTERVAL.clamp(b) * 256f64) as u8;
    [r, g, b]
}
