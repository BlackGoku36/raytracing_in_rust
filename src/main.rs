pub mod raytrace;

use raytrace::vec::Vec3;
use raytrace::ray::Ray;
use raytrace::sphere::Sphere;

use raytrace::hitable::Hitable;
use raytrace::hitable_list::HitableList;
use raytrace::camera::Camera;
use raytrace::vec::drand48;

use raytrace::material::Lambertian;
use raytrace::material::Metal;
use raytrace::material::Dielectric;

fn color(r: Ray, world: &HitableList, depth: i32) -> Vec3{
    match world.hit(r, 0.001, std::f32::MAX) {
        Some(rec) => {
            let mut scattered: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation:Vec3 = Vec3::new(0.0, 0.0, 0.0);
            if depth < 50 && rec.material.scatter(&r, &rec, &mut attenuation, &mut scattered){
                attenuation * color(scattered, world, depth + 1)
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

//------- Random Scene --------
fn random_scene() -> HitableList {
    let n = 500;
    let mut world = HitableList::new(n + 1);
    world.add(
        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 
        1000.0, 
        Box::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)))))
    );
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center-Vec3::new(4.0, 0.2, 0.0)).length()>0.9{
                if choose_mat < 0.8 {
                    world.add(
                        Box::new(Sphere::new(center,
                        0.2, 
                        Box::new(Lambertian::new(Vec3::new(drand48()*drand48(), drand48()*drand48(), drand48()*drand48())))))
                    );

                } else if choose_mat < 0.95 {
                    world.add(
                        Box::new(Sphere::new(center, 
                        0.2, 
                        Box::new(Metal::new(Vec3::new(0.5 * (1.0 + drand48()), 0.5 * ( 1.0 + drand48()), 0.5 * (1.0 + drand48())), 0.5 * ( 1.0 + drand48())))))
                    );

                } else {
                    world.add(
                        Box::new(Sphere::new(center, 
                        0.2, 
                        Box::new(Dielectric::new(1.5))))
                    );
                }
            }
        }
    }
    world.add(
        Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 
        1.0, 
        Box::new(Dielectric::new(1.5))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 
        1.0, 
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 
        1.0, 
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0))))
    );
    
    world
}
//--------

//------ Default Scene with 4 Sphere ------
fn default_scene() -> HitableList {
    let mut world = HitableList::new(4);
    world.add(
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 
        0.5, 
        Box::new(Lambertian::new(Vec3::new(0.2, 0.2, 0.8)))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 
        100.0, Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 
        0.5, 
        Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 
        0.5, 
        Box::new(Dielectric::new(1.5))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 
        -0.45, Box::new(Dielectric::new(1.5))))
    );

    world
}
//-------

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 10;
    print!("P3\n{} {}\n255\n", nx, ny);

    let look_from:Vec3 = Vec3::new(13.0, 2.0, 3.0);
    let look_at:Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let aperature:f32 = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        nx as f32/ny as f32,
        aperature,
        dist_to_focus
    );

    let world = random_scene();
    
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
