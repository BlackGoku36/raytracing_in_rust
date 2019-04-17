pub mod raytrace;
pub mod scenes;

use raytrace::ray::Ray;
use raytrace::vec::Vec3;

use raytrace::camera::Camera;
use raytrace::hitable::Hitable;
use raytrace::hitable_list::HitableList;
use raytrace::vec::drand48;

use rayon::prelude::*;

use scenes::{
    // lighted_perlin_spheres::lightted_perlin_spheres,
    // random_spheres::{
    //     random_scene,
    //     moving_random_scene
    // },
    // cornell_box::{
    //     cornell_box, cornell_smoke
    // },
    final_scene::final_scene,
    // checkered_texture::checkered_texture_scene,
    // default_scene::default_scene,
    perlin_spheres::perlin_spheres,
    textured_sphere::textured_spheres,
};

fn color(r: Ray, world: &HitableList, depth: i32) -> Vec3 {
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            if depth >= 50 {
                return Vec3::new(0.0, 0.0, 0.0);
            }
            let emitted = rec.material.emitted(0.0, 0.0, rec.p);
            if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
                emitted + attenuation * color(scattered, world, depth + 1)
            } else {
                emitted
            }
        }
        None => {
            Vec3::new(0.0, 0.0, 0.0)
            // let unit_direction = Vec3::make_unit_vector(r.direction());
            // let t = 0.5 * (unit_direction.y() + 1.0);
            // return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn main() {
    let nx = 600;
    let ny = 600;
    let ns = 10000;
    print!("P3\n{} {}\n255\n", nx, ny);
    let look_from: Vec3 = Vec3::new(478.0, 278.0, -600.0);
    let look_at: Vec3 = Vec3::new(278.0, 278.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature: f32 = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        nx as f32 / ny as f32,
        aperature,
        dist_to_focus,
        0.0,
        1.0,
    );

    let world = final_scene();

    let rows: Vec<Vec<Vec3>> = (0..ny)
        .into_par_iter()
        .rev()
        .map(|j| {
            (0..nx)
                .into_par_iter()
                .map(|i| {
                    let mut col = Vec3::new(0.0, 0.0, 0.0);
                    for _s in 0..ns {
                        let u = (i as f32 + drand48()) / nx as f32;
                        let v = (j as f32 + drand48()) / ny as f32;
                        let r = cam.get_ray(u, v);
                        col += color(r, &world, 0);
                    }
                    col /= ns as f32;
                    col = Vec3::new(f32::sqrt(col[0]), f32::sqrt(col[1]), f32::sqrt(col[2]));
                    col *= 255.99;
                    col
                })
                .collect()
        })
        .collect();

    for r in rows {
        for col in r {
            print!("{} {} {}\n", col.r() as i32, col.g() as i32, col.b() as i32);
        }
    }
}
