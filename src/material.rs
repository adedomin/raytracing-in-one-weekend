use crate::{
    hit::HitRec,
    ray::Ray,
    vec3::{Vec3, safe_random_unit_vec},
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
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRec) -> Option<Scatter>;
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
        }
    }
}
