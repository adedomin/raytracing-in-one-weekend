use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{hit::HitRange, render::RGB};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub const ZERO: Vec3 = Vec3(0., 0., 0.);
    pub const ONE: Vec3 = Vec3(1., 1., 1.);

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

impl From<Vec3> for RGB {
    fn from(value: Vec3) -> Self {
        let Vec3(r, g, b) = value;
        let r = (COLOR_INTERVAL.clamp(r) * 256f64) as u8;
        let g = (COLOR_INTERVAL.clamp(g) * 256f64) as u8;
        let b = (COLOR_INTERVAL.clamp(b) * 256f64) as u8;
        [r, g, b]
    }
}
