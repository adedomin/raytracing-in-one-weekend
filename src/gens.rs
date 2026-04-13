#![allow(unused)]

use std::f64;

use crate::{
    hit::{self, Hittable},
    ray::Ray,
    render::{Image, RGB},
    shapes::Sphere,
    vec3::Vec3,
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

fn p4_ray_color(ray: Ray) -> RGB {
    let unit_dir = ray.dir.unit_vector();
    let a = 0.5 * (unit_dir.y() + 1.0);
    ((1.0 - a) * Vec3::ONE + a * Vec3(0.5, 0.7, 1.0)).into()
}

fn p5_ray_color(ray: Ray) -> RGB {
    let sphere = Sphere::new(Vec3(0., 0., -1.0), 0.5);
    if let Some(hit) = sphere.hit(&ray, &hit::lim::ZERO_TO) {
        let surface_p = (hit.p - Vec3(0., 0., -1.)).unit_vector();
        (0.5 * (surface_p + Vec3::ONE)).into()
    } else {
        p4_ray_color(ray)
    }
}

fn p6_ray_color<T: Hittable>(ray: Ray, world: &T) -> RGB {
    if let Some(hit) = world.hit(&ray, &hit::lim::ZERO_TO) {
        (0.5 * (hit.norm + Vec3::ONE)).into()
    } else {
        p4_ray_color(ray)
    }
}

pub fn p4() -> Image {
    let img_w = 400f64;
    let img_h = (img_w / WIDE_RATIO).clamp(1f64, u32::MAX as f64).trunc();

    let world = vec![
        Sphere::new(Vec3(0., 0., -1.), 0.5),
        Sphere::new(Vec3(0., -100.5, -1.), 100.),
    ];

    let camera = Vec3::ZERO;
    let focal = 1f64;

    let view_h = 2f64;
    let view_w = view_h * (img_w / img_h);
    let view_u = Vec3(view_w, 0f64, 0f64);
    let view_v = Vec3(0f64, -view_h, 0f64);

    let pix_u_delta = view_u / img_w;
    let pix_v_delta = view_v / img_h;

    let view_left = camera - Vec3(0.0, 0.0, focal) - (view_u / 2.0) - (view_v / 2.0);
    let pix_0 = view_left + (0.5 * (pix_u_delta + pix_v_delta));

    let rend = xyrange(0, img_w as u32, 0, img_h as u32)
        .map(|(w, h)| {
            let w = w as f64;
            let h = h as f64;
            let pix_center = pix_0 + (w * pix_u_delta) + (h * pix_v_delta);
            p6_ray_color(
                Ray {
                    orig: camera,
                    dir: pix_center - camera,
                },
                &world,
            )
        })
        .collect();
    Image::new(rend, img_w as u32, img_h as u32)
}
