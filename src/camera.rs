use rayon::iter::{IntoParallelRefIterator as _, ParallelIterator as _};

use crate::{hit::Hittable, ray::Ray, render::Image, vec3::Vec3};

fn xyrange(sx: u32, ex: u32, sy: u32, ey: u32) -> impl Iterator<Item = (u32, u32)> {
    (sy..ey).flat_map(move |j| (sx..ex).map(move |i| (i, j)))
}

#[derive(Clone)]
pub struct Camera {
    aspect_ratio: f64,
    width: f64,
    height: f64,
    center: Vec3,
    // Pixel (0, 0) location
    pix_orig: Vec3,
    // Pixel Δu
    p_d_u: Vec3,
    // Pixel Δv
    p_d_v: Vec3,
    samples_pp: u8,
    ray_recurse: u8,
}

impl Camera {
    pub fn new(center: Vec3, aspect_ratio: f64, width: f64) -> Self {
        let height = (width / aspect_ratio).clamp(1., u32::MAX as f64).trunc();
        let focal = 1f64;

        let view_h = 2f64;
        let view_w = view_h * (width / height);
        let view_u = Vec3(view_w, 0f64, 0f64);
        let view_v = Vec3(0f64, -view_h, 0f64);
        let view_left = center - Vec3(0.0, 0.0, focal) - (view_u / 2.0) - (view_v / 2.0);
        let p_d_u = view_u / width;
        let p_d_v = view_v / height;
        let pix_orig = view_left + (0.5 * (p_d_u + p_d_v));

        Self {
            aspect_ratio,
            width,
            height,
            center,
            pix_orig,
            p_d_u,
            p_d_v,
            samples_pp: 100,
            ray_recurse: 50,
        }
    }

    fn get_rand_ray(&self, w: f64, h: f64) -> Ray {
        const RAND_RANGE: std::ops::Range<f64> = -0.5..(0.5 + f64::EPSILON);
        let w_r = rand::random_range(RAND_RANGE);
        let h_r = rand::random_range(RAND_RANGE);
        let pix_sample = self.pix_orig + ((w + w_r) * self.p_d_u) + ((h + h_r) * self.p_d_v);
        Ray {
            orig: self.center,
            dir: pix_sample - self.center,
        }
    }

    pub fn render<T: Hittable + Send + Sync>(
        &self,
        world: &T,
        shader: impl Fn(Ray, &T, u8) -> Vec3 + Send + Sync,
    ) -> Image {
        let xys = xyrange(0, self.width as u32, 0, self.height as u32).collect::<Vec<_>>();
        Image::new(
            xys.par_iter()
                .map(|(w, h)| {
                    let w = *w as f64;
                    let h = *h as f64;
                    let sample_scaled = 1.0 / self.samples_pp as f64;
                    let sampled = (0..self.samples_pp)
                        .map(|_| shader(self.get_rand_ray(w, h), world, self.ray_recurse))
                        .fold(Vec3::ZERO, |acc, v| acc + v);
                    (sampled * sample_scaled).into()
                })
                .collect(),
            self.width as u32,
            self.height as u32,
        )
    }
}
