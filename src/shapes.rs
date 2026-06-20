use glam::DVec3;

use crate::{
    aabb::{self, AABB},
    hit::{HitRange, HitRec, Hittable},
    ray::Ray,
};

pub struct Sphere {
    center: DVec3,
    rad: f64,
}

impl Sphere {
    pub fn new(center: DVec3, rad: f64) -> Self {
        Sphere { center, rad }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_lim: &HitRange) -> Option<HitRec> {
        let partial_c = self.center - r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(partial_c);
        let c = partial_c.length_squared() - self.rad.powi(2);
        let discrim = h.powi(2) - a * c;
        (discrim >= 0.)
            .then(|| {
                let sq = discrim.sqrt();
                ((h - sq) / a, (h + sq) / a)
            })
            .and_then(|(t1, t2)| {
                t_lim
                    .surrounds(t1)
                    .then_some(t1)
                    .or(t_lim.surrounds(t2).then_some(t2))
            })
            .map(move |t| {
                let p = r.at(t);
                let norm = (p - self.center) / self.rad;
                HitRec::new_gen_face(r, t, p, norm)
            })
    }

    fn bounding_box(&self) -> AABB {
        let rect_vec = DVec3::splat(self.rad);
        AABB::from((self.center - rect_vec, self.center + rect_vec))
    }
}

macro_rules! impl_hittable_array_like {
    ($ty:ty) => {
        impl<T: Hittable> Hittable for $ty {
            fn hit(&self, r: &Ray, t_lim: &HitRange) -> Option<HitRec> {
                self.iter()
                    .enumerate()
                    .fold((t_lim.end, None), |(tmax, h), (i, curr)| {
                        curr.hit(r, &(t_lim.start..tmax).into())
                            .map(|h| (h.t, Some(h.set_ancillary(i))))
                            .unwrap_or((tmax, h))
                    })
                    .1
            }

            fn bounding_box(&self) -> AABB {
                self.iter()
                    .map(|el| el.bounding_box())
                    .reduce(|acc, el| AABB::from((acc, el)))
                    .unwrap_or(aabb::EMPTY)
            }
        }
    };
}

impl_hittable_array_like!(Vec<T>);
impl_hittable_array_like!(&[T]);
impl_hittable_array_like!(&mut [T]);

impl<H: Hittable, M> Hittable for (H, M) {
    fn hit(&self, r: &Ray, t_lim: &HitRange) -> Option<HitRec> {
        self.0.hit(r, t_lim)
    }

    fn bounding_box(&self) -> AABB {
        self.0.bounding_box()
    }
}
