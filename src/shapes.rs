use std::ops::RangeInclusive;

use crate::{
    hit::{HitRec, Hittable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    rad: f64,
}

impl Sphere {
    pub fn new(center: Vec3, rad: f64) -> Self {
        Sphere { center, rad }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_lim: &RangeInclusive<f64>) -> Option<HitRec> {
        let partial_c = self.center - r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(partial_c);
        let c = partial_c.length_squared() - self.rad.powi(2);
        let discrim = h.powi(2) - a * c;
        (discrim >= 0.)
            .then(|| (h - discrim.sqrt()) / a)
            .filter(|t| t_lim.contains(t))
            .map(move |t| {
                let p = r.at(t);
                let norm = (p - self.center) / self.rad;
                HitRec::new_gen_face(r, t, p, norm)
            })
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, r: &Ray, t_lim: &RangeInclusive<f64>) -> Option<HitRec> {
        // self.iter()
        //     .flat_map(|h| h.hit(r, t_lim))
        //     .reduce(|h1, h2| if h1.t <= h2.t { h1 } else { h2 })
        self.iter()
            .fold((*t_lim.end(), None), |(tmax, h), curr| {
                curr.hit(r, &(*t_lim.start()..=tmax))
                    .map(|h| (h.t, Some(h)))
                    .unwrap_or((tmax, h))
            })
            .1
    }
}
