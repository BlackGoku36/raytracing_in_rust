fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b: f32 = 0.2;
            let ir = (255.0 * r) as i32;
            let ig = (255.0 * g) as i32;
            let ib = (255.0 * b) as i32;
            print!("{} {} {}\n", ir, ig, ib );
        }
    }
}