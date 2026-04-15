use std::ops::Neg;

use crate::{
    hit::HitRec,
    ray::Ray,
    vec3::{Vec3, rand_double, safe_random_unit_vec},
};

pub struct Scatter {
    pub attenuation: Vec3,
    pub dir: Ray,
}

#[derive(Clone, Copy)]
pub enum Mats {
    None,
    Lambertian(Vec3),
    Metal(Vec3, f64),
    Mirror(Vec3),
    Dielectric(f64),
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRec) -> Option<Scatter>;
}

fn reflectance(cos: f64, refract: f64) -> f64 {
    let r0 = ((1. - refract) / (1. + refract)).powi(2);
    r0 + (1. - r0) * (1. - cos).powi(5)
}

impl Material for Mats {
    fn scatter(&self, ray: &Ray, hit: &HitRec) -> Option<Scatter> {
        match self {
            Mats::None => None,
            Mats::Lambertian(albedo) => {
                let dir = hit.norm + safe_random_unit_vec();
                let dir = if dir.near_zero() { hit.norm } else { dir };
                Some(Scatter {
                    attenuation: *albedo,
                    dir: Ray { orig: hit.p, dir },
                })
            }
            Mats::Metal(albedo, fuzz) => {
                let fuzz = fuzz.clamp(0.0, 1.0);
                let dir = ray.dir.reflect(hit.norm);
                let dir = dir.unit_vector() + (fuzz * safe_random_unit_vec());
                (dir.dot(hit.norm) > 0.).then_some(Scatter {
                    attenuation: *albedo,
                    dir: Ray { orig: hit.p, dir },
                })
            }
            Mats::Mirror(albedo) => {
                let dir = ray.dir.reflect(hit.norm);
                Some(Scatter {
                    attenuation: *albedo,
                    dir: Ray { orig: hit.p, dir },
                })
            }
            Mats::Dielectric(refraction) => {
                let eta = if hit.face {
                    1.0 / refraction
                } else {
                    *refraction
                };

                let unit_d = ray.dir.unit_vector();
                let cos_theta = unit_d.neg().dot(hit.norm).min(1.0);
                let sin_theta = (1. - cos_theta.powi(2)).sqrt();

                let cannot_refract = eta * sin_theta > 1.;

                let dir = if cannot_refract || reflectance(cos_theta, eta) > rand_double() {
                    unit_d.reflect(hit.norm)
                } else {
                    unit_d.refract(hit.norm, eta)
                };

                Some(Scatter {
                    attenuation: Vec3::ONE,
                    dir: Ray { orig: hit.p, dir },
                })
            }
        }
    }
}
