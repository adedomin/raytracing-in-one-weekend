use rand::random_range;
use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

use crate::{hit::Hittable, ray::Ray, render::Image, vec3::Vec3};

fn xyrange(sx: u32, ex: u32, sy: u32, ey: u32) -> impl Iterator<Item = (u32, u32)> {
    (sy..ey).flat_map(move |j| (sx..ex).map(move |i| (i, j)))
}

#[derive(Clone)]
pub struct Camera {
    width: f64,
    height: f64,
    center: Vec3,
    // Pixel (0, 0) location
    pix_orig: Vec3,
    // Pixel Δu
    p_d_u: Vec3,
    // Pixel Δv
    p_d_v: Vec3,
    samples_pp: u32,
    ray_recurse: u8,
    defocus_u: Vec3,
    defocus_v: Vec3,
    defocus_angle: f64,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        width: f64,
        fov: f64,
        focal: f64,
        defocus_angle: f64,
        samples_pp: u32,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
    ) -> Self {
        let height = (width / aspect_ratio).clamp(1., u32::MAX as f64).trunc();

        let theta = fov.to_radians();
        let h = (theta / 2.).tan();

        let view_h = 2. * h * focal;
        let view_w = view_h * (width / height);

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
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

    fn defocus_sample(&self) -> Vec3 {
        let p = loop {
            let p = Vec3(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.);
            if p.length_squared() < 1. {
                break p;
            }
        };
        self.center + (p.0 * self.defocus_u) + (p.1 * self.defocus_v)
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
        shader: impl Fn(Ray, &T, u8) -> Vec3 + Send + Sync,
    ) -> Image {
        let xys = xyrange(0, self.width as u32, 0, self.height as u32).collect::<Vec<_>>();
        #[cfg(feature = "progress")]
        let tx = {
            let (tx, rx) = std::sync::mpsc::channel();
            let xys_len = xys.len();
            std::thread::spawn(move || {
                let bar = indicatif::ProgressBar::new(xys_len as u64).with_style(
                    indicatif::ProgressStyle::with_template(
                        "T {elapsed_precise} / ETA {eta_precise} {wide_bar} ({pos}/{len})",
                    )
                    .unwrap(),
                );
                while let Ok(false) = rx.recv() {
                    bar.inc(1);
                }
                bar.finish();
            });
            tx
        };
        let img = xys
            .par_iter()
            .map(|(w, h)| {
                let w = *w as f64;
                let h = *h as f64;
                let sample_scaled = 1.0 / self.samples_pp as f64;
                let sampled = (0..self.samples_pp)
                    .map(|_| shader(self.get_rand_ray(w, h), world, self.ray_recurse))
                    .fold(Vec3::ZERO, |acc, v| acc + v);
                let ret = (sampled * sample_scaled).into();
                #[cfg(feature = "progress")]
                {
                    _ = tx.send(false);
                }
                ret
            })
            .collect();
        #[cfg(feature = "progress")]
        {
            _ = tx.send(true).unwrap();
        }
        Image::new(img, self.width as u32, self.height as u32)
    }
}
