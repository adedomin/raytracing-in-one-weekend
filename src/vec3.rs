use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{hit::HitRange, render::RGB};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::splat(0.);
    pub const ONE: Vec3 = Vec3::splat(1.);

    pub const fn x(&self) -> f64 {
        self.0
    }
    pub const fn y(&self) -> f64 {
        self.1
    }
    pub const fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(self, rhs: Self) -> f64 {
        let inter = self * rhs;
        inter.0 + inter.1 + inter.2
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit_vector(self) -> Self {
        let len = self.length();
        self / len
    }

    pub fn near_zero(&self) -> bool {
        const NEAR: std::ops::Range<f64> = -1e-8f64..1e-8f64;
        NEAR.contains(&self.0) && NEAR.contains(&self.1) && NEAR.contains(&self.2)
    }

    pub fn reflect(self, rhs: Self) -> Self {
        self - 2. * self.dot(rhs) * rhs
    }

    pub fn refract(self, rhs: Self, etai_over_etat: f64) -> Self {
        let cos_theta = self.neg().dot(rhs).min(1.0);
        let r_out_perp = etai_over_etat * (self + cos_theta * rhs);
        let r_out_par = (1.0 - r_out_perp.length_squared()).abs().sqrt().neg() * rhs;
        r_out_perp + r_out_par
    }

    pub const fn splat(s: f64) -> Self {
        Vec3(s, s, s)
    }
}

pub fn rand_double() -> f64 {
    rand::random_range(0.0..1.)
}

pub fn rand_vec3_range(r: std::ops::Range<f64>) -> Vec3 {
    Vec3(
        rand::random_range(r.clone()),
        rand::random_range(r.clone()),
        rand::random_range(r),
    )
}

pub fn rand_vec3() -> Vec3 {
    rand_vec3_range(0.0..1.)
}

pub fn safe_random_unit_vec() -> Vec3 {
    loop {
        let v = rand_vec3_range(-1.0..1.);
        let lensq = v.length_squared();
        if 1.0e-160 < lensq && lensq <= 1. {
            return v / lensq.sqrt();
        }
    }
}

pub fn rand_on_hemi(norm: Vec3) -> Vec3 {
    let r = safe_random_unit_vec();
    if r.dot(norm) > 0.0 { r } else { -r }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

const COLOR_INTERVAL: HitRange = HitRange::new(0., 0.999);

fn lin_to_srgb_gamma(lin: f64) -> f64 {
    if lin <= 0.0031308 {
        12.92 * lin
    } else {
        1.055 * lin.powf(1. / 2.4) - 0.055
    }
}

impl From<Vec3> for RGB {
    fn from(value: Vec3) -> Self {
        let Vec3(r, g, b) = value;
        let r = lin_to_srgb_gamma(r);
        let g = lin_to_srgb_gamma(g);
        let b = lin_to_srgb_gamma(b);

        let r = (COLOR_INTERVAL.clamp(r) * 256f64) as u8;
        let g = (COLOR_INTERVAL.clamp(g) * 256f64) as u8;
        let b = (COLOR_INTERVAL.clamp(b) * 256f64) as u8;
        [r, g, b]
    }
}
