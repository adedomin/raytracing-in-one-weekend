#![allow(unused)]

use std::f64;

use glam::DVec3;
use rand::random_range;

use crate::{
    bvh::Bvh,
    camera::Camera,
    hit::{self, Hittable},
    material::{Material, Mats, Scatter},
    ray::Ray,
    render::{Image, RGB},
    shapes::Sphere,
    vec_help::{rand_double, rand_on_hemi, rand_vec3, safe_random_unit_vec, to_srgb},
};

fn xyrange(sx: i32, ex: i32, sy: i32, ey: i32) -> impl Iterator<Item = (i32, i32)> {
    (sy..ey).flat_map(move |j| (sx..ex).map(move |i| (i, j)))
}

pub fn initial() -> Image {
    let width = 256;
    let height = 256;
    let rend = xyrange(0, width, 0, height)
        .map(|(w, h)| {
            to_srgb(DVec3::new(
                (w as f64) / (width as f64 - 1f64),
                (h as f64) / (height as f64 - 1f64),
                0.0,
            ))
        })
        .collect();
    Image::new(rend, width as u32, height as u32)
}

const WIDE_RATIO: f64 = 16f64 / 9f64;

fn p4_ray_color(ray: Ray) -> DVec3 {
    let unit_dir = ray.dir.normalize();
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * DVec3::ONE + a * DVec3::new(0.5, 0.7, 1.0)
}

fn p5_ray_color(ray: Ray) -> DVec3 {
    let sphere = (Sphere::new(DVec3::new(0., 0., -1.0), 0.5), Mats::None);
    if let Some(hit) = Hittable::hit(&sphere, &ray, &hit::lim::ZERO_TO) {
        let surface_p = (hit.p - DVec3::new(0., 0., -1.)).normalize();
        0.5 * (surface_p + DVec3::ONE)
    } else {
        p4_ray_color(ray)
    }
}

fn p6_ray_color<T: Hittable>(ray: Ray, world: &T) -> DVec3 {
    if let Some(hit) = world.hit(&ray, &hit::lim::ZERO_TO) {
        0.5 * (hit.norm + DVec3::ONE)
    } else {
        p4_ray_color(ray)
    }
}

fn p9_ray_color<T: Hittable>(ray: Ray, world: &T, depth: u8) -> DVec3 {
    if depth == 0 {
        return DVec3::ZERO;
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

fn p10_ray_color<T: Hittable + Material>(mut ray: Ray, world: &T, depth: u8) -> DVec3 {
    let mut atn = DVec3::ONE;
    for _ in 0..depth {
        if let Some(hit) = world.hit(&ray, &(0.001f64..).into()) {
            if let Some(Scatter { attenuation, dir }) = world.scatter(&ray, &hit) {
                ray = dir;
                atn *= attenuation;
            } else {
                return DVec3::ZERO;
            }
        } else {
            return atn * p4_ray_color(ray);
        }
    }
    DVec3::ZERO
}

// pub fn p4() -> Image {
//     let camera = Camera::builder()
//         .aspect_ratio(WIDE_RATIO)
//         .width(1080.)
//         .fov(20.)
//         .focal(3.4)
//         .defocus_angle(10.)
//         .samples_pp(500)
//         .lookfrom(DVec3::new(-2., 2., 1.))
//         .lookat(DVec3::NEG_Z)
//         .vup(DVec3::Y)
//         .build();
//     let ground_mat = Mats::Lambertian(DVec3::new(0.8, 0.8, 0.0));
//     let mat_cent = Mats::Lambertian(DVec3::new(0.1, 0.2, 0.5));
//     let mat_left = Mats::Dielectric(1.5);
//     let mat_left_bubble = Mats::Dielectric(1.0 / 1.5);
//     let mat_right = Mats::Metal(DVec3::new(0.8, 0.6, 0.2), 1.0);

//     let world = vec![
//         (Sphere::new(DVec3::new(0., -100.5, -1.), 100.), ground_mat),
//         (Sphere::new(DVec3::new(0., 0., -1.2), 0.5), mat_cent),
//         (Sphere::new(DVec3::new(-1., 0., -1.), 0.5), mat_left),
//         (Sphere::new(DVec3::new(-1., 0., -1.), 0.4), mat_left_bubble),
//         (Sphere::new(DVec3::new(1., 0., -1.), 0.5), mat_right),
//     ];
//     // let r = (f64::consts::PI / 4.).cos();
//     // let world = vec![
//     //     Sphere::new(DVec3(-r, 0., -1.), r, Mats::Lambertian(DVec3(0., 0., 1.))),
//     //     Sphere::new(DVec3(r, 0., -1.), r, Mats::Lambertian(DVec3(1., 0., 0.))),
//     // ];

//     camera.render(|r, d| p10_ray_color(r, &world, d))
// }

pub fn final_rand() -> Image {
    let camera = Camera::builder()
        .aspect_ratio(WIDE_RATIO)
        .width(1200.)
        .fov(20.)
        .focal(10.)
        .defocus_angle(0.6)
        .samples_pp(500)
        .lookfrom(DVec3::new(13., 2., 3.))
        .lookat(DVec3::ZERO)
        .vup(DVec3::Y)
        .build();
    let ground_mat = Mats::Lambertian(DVec3::new(0.5, 0.5, 0.5));
    let mut world = vec![(Sphere::new(DVec3::new(0., -1000., 0.), 1000.), ground_mat)];

    xyrange(-11, 11, -11, 11).for_each(|(b, a)| {
        let b = b as f64;
        let a = a as f64;
        let choose = rand_double();
        let center = DVec3::new(a + 0.9 * rand_double(), 0.2, b + 0.9 * rand_double());
        if (center - DVec3::new(4., 0.2, 0.)).length() > 0.9 {
            if choose < 0.8 {
                // diffuse
                let albedo = rand_vec3() * rand_vec3();
                world.push((Sphere::new(center, 0.2), Mats::Lambertian(albedo)));
            } else if choose < 0.95 {
                let albedo = rand_vec3();
                let fuzz = random_range(0.0..0.5);
                world.push((Sphere::new(center, 0.2), Mats::Metal(albedo, fuzz)));
            } else {
                world.push((Sphere::new(center, 0.2), Mats::Dielectric(1.5)));
            }
        }
    });
    world.push((
        Sphere::new(DVec3::new(0., 1., 0.), 1.),
        Mats::Dielectric(1.5),
    ));
    world.push((
        Sphere::new(DVec3::new(-4., 1., 0.), 1.),
        Mats::Lambertian(DVec3::new(0.4, 0.2, 0.1)),
    ));
    world.push((
        Sphere::new(DVec3::new(4., 1., 0.), 1.),
        Mats::Metal(DVec3::new(0.7, 0.6, 0.5), 0.0),
    ));
    let bvh_wrld = Bvh::new(&mut world);
    camera.render(&bvh_wrld, p10_ray_color)
}
