use std::hash::{BuildHasher, RandomState};

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
}

// from [0.0, 1.0)
fn rand_f64() -> f64 {
    // u32 should fit in an f64
    let rand = RandomState::new().hash_one(()) % u32::MAX as u64;
    rand as f64 / (u32::MAX as f64 + 1f64)
}

fn rand_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
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
        }
    }

    fn get_rand_ray(&self, w: f64, h: f64) -> Ray {
        let w_r = rand_f64() - 0.5;
        let h_r = rand_f64() - 0.5;
        let pix_sample = self.pix_orig + ((w + w_r) * self.p_d_u) + ((h + h_r) * self.p_d_v);
        Ray {
            orig: self.center,
            dir: pix_sample - self.center,
        }
    }

    pub fn render<T: Hittable>(&self, world: &T, shader: impl Fn(Ray, &T) -> Vec3) -> Image {
        Image::new(
            xyrange(0, self.width as u32, 0, self.height as u32)
                .map(|(w, h)| {
                    let w = w as f64;
                    let h = h as f64;
                    let sample_scaled = 1.0 / self.samples_pp as f64;
                    let sampled = (0..self.samples_pp)
                        .map(|_| shader(self.get_rand_ray(w, h), world))
                        .fold(Vec3::ZERO, |acc, v| acc + v);
                    (sampled * sample_scaled).into()
                })
                .collect(),
            self.width as u32,
            self.height as u32,
        )
    }
}
