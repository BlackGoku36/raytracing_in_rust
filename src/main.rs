pub mod raytrace;

use raytrace::vec::Vec3;
use raytrace::ray::Ray;

fn hit_sphere(center: Vec3, radius: f32, r: Ray)-> f32{
    let oc:Vec3 = r.origin() - center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0*Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0{
        -1.0
    }else{
        (-b-f32::sqrt(discriminant))/(2.0*a)
    }
}

fn color(r:Ray)-> Vec3{
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0{
        let n = Vec3::make_unit_vector(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5*Vec3::new(n.x()+1.0, n.y()+1.0, n.z()+1.0);
    }
    let unit_direction = Vec3::make_unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = color(r);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            print!("{} {} {}\n", ir, ig, ib );
        }
    }
}
