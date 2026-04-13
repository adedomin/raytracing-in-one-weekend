#![allow(unused)]

use std::f64;

use crate::{
    camera::Camera,
    hit::{self, Hittable},
    ray::Ray,
    render::{Image, RGB},
    shapes::Sphere,
    vec3::{Vec3, rand_on_hemi, safe_random_unit_vec},
};

fn xyrange(sx: u32, ex: u32, sy: u32, ey: u32) -> impl Iterator<Item = (u32, u32)> {
    (sy..ey).flat_map(move |j| (sx..ex).map(move |i| (i, j)))
}

pub fn initial() -> Image {
    let width = 256;
    let height = 256;
    let rend = xyrange(0, width, 0, height)
        .map(|(w, h)| {
            Vec3(
                (w as f64) / (width as f64 - 1f64),
                (h as f64) / (height as f64 - 1f64),
                0.0,
            )
            .into()
        })
        .collect();
    Image::new(rend, width, height)
}

const WIDE_RATIO: f64 = 16f64 / 9f64;

fn p4_ray_color(ray: Ray) -> Vec3 {
    let unit_dir = ray.dir.unit_vector();
    let a = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - a) * Vec3::ONE + a * Vec3(0.5, 0.7, 1.0)
}

fn p5_ray_color(ray: Ray) -> Vec3 {
    let sphere = Sphere::new(Vec3(0., 0., -1.0), 0.5);
    if let Some(hit) = sphere.hit(&ray, &hit::lim::ZERO_TO) {
        let surface_p = (hit.p - Vec3(0., 0., -1.)).unit_vector();
        0.5 * (surface_p + Vec3::ONE)
    } else {
        p4_ray_color(ray)
    }
}

fn p6_ray_color<T: Hittable>(ray: Ray, world: &T) -> Vec3 {
    if let Some(hit) = world.hit(&ray, &hit::lim::ZERO_TO) {
        0.5 * (hit.norm + Vec3::ONE)
    } else {
        p4_ray_color(ray)
    }
}

fn p9_ray_color<T: Hittable>(ray: Ray, world: &T, depth: u8) -> Vec3 {
    if depth == 0 {
        return Vec3::ZERO;
    }

    // 0.001 to reduce shadow acne.
    if let Some(hit) = world.hit(&ray, &(0.001f64..).into()) {
        // let new_rand_ray_dir = rand_on_hemi(hit.norm);
        let new_rand_ray_dir = hit.norm + safe_random_unit_vec();
        let new_ray = Ray {
            orig: hit.p,
            dir: new_rand_ray_dir,
        };
        0.1 * p9_ray_color(new_ray, world, depth - 1)
    } else {
        p4_ray_color(ray)
    }
}

pub fn p4() -> Image {
    let camera = Camera::new(Vec3::ZERO, WIDE_RATIO, 400.);

    let world = vec![
        Sphere::new(Vec3(0., 0., -1.), 0.5),
        Sphere::new(Vec3(0., -100.5, -1.), 100.),
    ];

    camera.render(&world, p9_ray_color)
}
