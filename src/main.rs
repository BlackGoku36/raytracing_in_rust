pub mod raytrace;

use raytrace::vec::Vec3;
use raytrace::ray::Ray;
use raytrace::hitable::Hitable;
use raytrace::hitable_list::HitableList;
use raytrace::sphere::Sphere;
use raytrace::camera::Camera;

extern crate rand;
use rand::Rng;

fn color(r: Ray, world: &HitableList)-> Vec3{
    match world.hit(r, 0.0, std::f32::MAX) {
        Some(rec) => {
            return 0.5*Vec3::new(rec.normal.x()+1.0, rec.normal.y()+1.0, rec.normal.z()+1.0);
        }
        None => {
            let unit_direction = Vec3::make_unit_vector(r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            return (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
        }
    }
}

fn drand48()->f32{
    let random_float: f32 = rand::thread_rng().gen();
    random_float
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 50;
    print!("P3\n{} {}\n255\n", nx, ny);

    let cam = Camera::new(Vec3::new(0.0, 0.0, 0.0));

    let mut world = HitableList::new(2);

    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..ns {
                let u = (i as f32 + drand48()) / nx as f32;
                let v = (j as f32 + drand48()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(r, &world);
            }
            col /= ns as f32;
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            print!("{} {} {}\n", ir, ig, ib );
        }
    }
}
