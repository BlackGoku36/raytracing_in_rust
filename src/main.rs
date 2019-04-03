pub mod raytrace;

use raytrace::vec::Vec3;

fn main() {
    let nx: u16 = 200;
    let ny: u16 = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            print!("{} {} {}\n", ir, ig, ib );
        }
    }
}
