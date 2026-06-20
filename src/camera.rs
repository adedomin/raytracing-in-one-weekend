use glam::DVec3;
use rand::random_range;
use rayon::iter::{
    IndexedParallelIterator as _, IntoParallelRefIterator as _, ParallelIterator as _,
};

use crate::{hit::Hittable, ray::Ray, render::Image, vec_help::to_srgb};

fn xyrange_expanded(sx: u32, ex: u32, sy: u32, ey: u32, repeat: u32) -> Vec<(u32, u32)> {
    let mut ret = Vec::with_capacity(((ex - sx) * (ey - sy) * repeat) as usize);
    for y in sy..ey {
        for x in sx..ex {
            ret.extend(std::iter::repeat_n((x, y), repeat as usize));
        }
    }
    ret
}

#[derive(Clone)]
pub struct Camera {
    width: f64,
    height: f64,
    center: DVec3,
    // Pixel (0, 0) location
    pix_orig: DVec3,
    // Pixel Δu
    p_d_u: DVec3,
    // Pixel Δv
    p_d_v: DVec3,
    samples_pp: u32,
    ray_recurse: u8,
    defocus_u: DVec3,
    defocus_v: DVec3,
    defocus_angle: f64,
}

#[bon::bon]
impl Camera {
    #[builder]
    pub fn new(
        aspect_ratio: f64,
        width: f64,
        fov: f64,
        focal: f64,
        defocus_angle: f64,
        samples_pp: u32,
        lookfrom: DVec3,
        lookat: DVec3,
        vup: DVec3,
    ) -> Self {
        let height = (width / aspect_ratio).clamp(1., u32::MAX as f64).trunc();

        let theta = fov.to_radians();
        let h = (theta / 2.).tan();

        let view_h = 2. * h * focal;
        let view_w = view_h * (width / height);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let view_u = view_w * u;
        let view_v = view_h * -v;

        let p_d_u = view_u / width;
        let p_d_v = view_v / height;

        let view_upper_left = lookfrom - (focal * w) - view_u / 2. - view_v / 2.;
        let pix_orig = view_upper_left + (0.5 * (p_d_u + p_d_v));

        let defocus_radius = focal * (defocus_angle / 2.).to_radians().tan();

        Self {
            width,
            height,
            center: lookfrom,
            pix_orig,
            p_d_u,
            p_d_v,
            samples_pp,
            ray_recurse: 50,
            defocus_angle,
            defocus_u: u * defocus_radius,
            defocus_v: v * defocus_radius,
        }
    }

    fn defocus_sample(&self) -> DVec3 {
        let p = loop {
            let p = DVec3::new(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.);
            if p.length_squared() < 1. {
                break p;
            }
        };
        self.center + (p.x * self.defocus_u) + (p.y * self.defocus_v)
    }

    fn get_rand_ray(&self, w: f64, h: f64) -> Ray {
        const RAND_RANGE: std::ops::Range<f64> = -0.5..(0.5 + f64::EPSILON);
        let w_r = rand::random_range(RAND_RANGE);
        let h_r = rand::random_range(RAND_RANGE);
        let pix_sample = self.pix_orig + ((w + w_r) * self.p_d_u) + ((h + h_r) * self.p_d_v);
        let orig = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_sample()
        };
        let dir = pix_sample - orig;
        Ray { orig, dir }
    }

    pub fn render<T: Hittable + Send + Sync>(
        &self,
        world: &T,
        shader: fn(Ray, &T, u8) -> DVec3,
    ) -> Image {
        // TODO: Fix this, 3 seconds!
        let xys = xyrange_expanded(0, self.width as u32, 0, self.height as u32, self.samples_pp);
        #[cfg(feature = "progress")]
        let bar = indicatif::ProgressBar::new(xys.len() as u64 / self.samples_pp as u64)
            .with_style(
                indicatif::ProgressStyle::with_template(
                    "T {elapsed_precise} / ETA {eta_precise} {wide_bar} ({pos}/{len})",
                )
                .unwrap(),
            );
        let img_iter = xys
            .par_iter()
            .map(|(w, h)| {
                let w = *w as f64;
                let h = *h as f64;
                shader(self.get_rand_ray(w, h), world, self.ray_recurse)
            })
            .fold_chunks_with(self.samples_pp as usize, DVec3::ZERO, |acc, v| acc + v)
            .map(|v| to_srgb(v / self.samples_pp as f64));
        #[cfg(feature = "progress")]
        let img_iter = img_iter.inspect(|_| {
            bar.inc(1);
        });
        let img = img_iter.collect();
        #[cfg(feature = "progress")]
        bar.abandon();
        Image::new(img, self.width as u32, self.height as u32)
    }
}
