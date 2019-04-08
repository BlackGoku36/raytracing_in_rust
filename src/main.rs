pub mod raytrace;
pub mod scenes;

use raytrace::vec::Vec3;
use raytrace::ray::Ray;

use raytrace::hitable::Hitable;
use raytrace::hitable_list::HitableList;
use raytrace::camera::Camera;
use raytrace::vec::drand48;

use scenes::{
    checkered_texture::checkered_texture_scene,
    default_scene::default_scene,
    random_spheres::{
        random_scene, 
        moving_random_scene
    },
};

fn color(r: Ray, world: &HitableList, depth: i32) -> Vec3{
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            if depth >= 50{
                return Vec3::new(0.0, 0.0, 0.0);
            }
            if let Some((scattered, attenuation)) = rec.material.scatter(&r, &rec) {
                attenuation * color(scattered, world, depth+1)
            }else{
                Vec3::new(0.0, 0.0, 0.0)
            }
        }
        None => {
            let unit_direction = Vec3::make_unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
        }
    }
}


fn main() {
    let nx = 200*1;
    let ny = 100*1;
    let ns = 10*1;
    print!("P3\n{} {}\n255\n", nx, ny);

    let look_from:Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let look_at:Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature:f32 = 0.0;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32/ny as f32,
        aperature,
        dist_to_focus,
        0.0, 
        1.0,
    );

    let world = checkered_texture_scene();
    
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f32 + drand48()) / nx as f32;
                let v = (j as f32 + drand48()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new( f32::sqrt(col[0]), f32::sqrt(col[1]), f32::sqrt(col[2]) );
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
