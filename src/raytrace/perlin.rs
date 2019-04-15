use super::vec::drand48;
use super::vec::Vec3;

pub struct Perlin {
    pub perm_x: Vec<usize>,
    pub perm_y: Vec<usize>,
    pub perm_z: Vec<usize>,
    pub ran_vec: Vec<Vec3>,
}

impl Perlin {
    pub fn new() -> Self {
        Perlin {
            perm_x: Perlin::perlin_generate_perm(),
            perm_y: Perlin::perlin_generate_perm(),
            perm_z: Perlin::perlin_generate_perm(),
            ran_vec: Perlin::perlin_generate(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f32 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum: f32 = 0.0;
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let c = self.ran_vec[(self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];
                    let di = di as f32;
                    let dj = dj as f32;
                    let dk = dk as f32;

                    let weight_v = Vec3::new(u - di, v - dj, w - dk);
                    accum += (di * uu + (1.0 - di) * (1.0 - uu))
                        * (dj * vv + (1.0 - dj) * (1.0 - vv))
                        * (dk * ww + (1.0 - dk) * (1.0 - ww))
                        * Vec3::dot(&c, &weight_v);
                }
            }
        }
        accum
    }

    pub fn perlin_generate() -> Vec<Vec3> {
        let mut p: Vec<Vec3> = Vec::with_capacity(256);
        for _i in 0..256 {
            p.push(Vec3::make_unit_vector(Vec3::new(
                -1.0 + 2.0 * drand48(),
                -1.0 + 2.0 * drand48(),
                -1.0 + 2.0 * drand48(),
            )));
        }
        p
    }

    pub fn perlin_generate_perm() -> Vec<usize> {
        let mut p: Vec<usize> = Vec::with_capacity(256);
        for i in 0..256 {
            p.push(i);
        }
        Perlin::permute(&mut p, 256);
        p
    }

    pub fn permute(p: &mut [usize], n: usize) {
        for i in (0..n).rev() {
            let target = (drand48() * (i + 1) as f32) as usize;
            p.swap(i, target);
        }
    }
}
