use crate::{
    hit::{HitRange, HitRec, Hittable},
    material::{Material, Mats},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere {
    center: Vec3,
    rad: f64,
    mat: Mats,
}

impl Sphere {
    pub fn new(center: Vec3, rad: f64, mat: Mats) -> Self {
        Sphere { center, rad, mat }
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
}

impl Material for Sphere {
    fn scatter(&self, ray: &Ray, hit: &HitRec) -> Option<crate::material::Scatter> {
        self.mat.scatter(ray, hit)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(&self, r: &Ray, t_lim: &HitRange) -> Option<HitRec> {
        //// enum all
        // self.iter()
        //     .flat_map(|h| h.hit(r, t_lim))
        //     .reduce(|h1, h2| if h1.t <= h2.t { h1 } else { h2 })

        //// This is (sort of) how the author does it.
        self.iter()
            .enumerate()
            .fold((t_lim.end, None), |(tmax, h), (i, curr)| {
                curr.hit(r, &(t_lim.start..tmax).into())
                    .map(|h| (h.t, Some(h.set_ancillary(i))))
                    .unwrap_or((tmax, h))
            })
            .1
    }
}

impl<T: Material> Material for Vec<T> {
    fn scatter(&self, ray: &Ray, hit: &HitRec) -> Option<crate::material::Scatter> {
        self[hit.ancillary].scatter(ray, hit)
    }
}
