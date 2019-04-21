use super::perlin::Perlin;
use super::vec::Vec3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
}

//---Constant Texture------
pub struct ConstantTexture {
    pub color: Vec3,
}

impl ConstantTexture {
    pub fn new(color: Vec3) -> Self {
        ConstantTexture { color }
    }
}

impl Texture for ConstantTexture {
    fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
        self.color
    }
}

//---Checker Texture------
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Box<Self> {
        Box::new(Self { odd, even })
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

//---Noise Texture-------
pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }

    fn turb(&self, p: Vec3) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        let depth: i32 = 7;
        for _i in 0..depth {
            accum += weight * self.noise.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.x() + 10.0 * self.turb(p)).sin())
    }
}

//---Image Texture------
pub struct ImageTexture {
    pixels: Vec<u8>,
    nx: u32,
    ny: u32,
}

impl ImageTexture {
    pub fn new(pixels: Vec<u8>, nx: u32, ny: u32) -> Self {
        ImageTexture { pixels, nx, ny }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Vec3) -> Vec3 {
        let nx = self.nx as usize;
        let ny = self.ny as usize;
        let mut i = (u * nx as f32) as usize;
        let mut j = ((1.0 - v) * ny as f32) as usize;
        if i > nx - 1 {
            i = nx - 1
        }
        if j > ny - 1 {
            j = ny - 1
        }
        let r = self.pixels[3 * i + 3 * nx * j] as f32 / 255.0;
        let g = self.pixels[3 * i + 3 * nx * j + 1] as f32 / 255.0;
        let b = self.pixels[3 * i + 3 * nx * j + 2] as f32 / 255.0;
        Vec3::new(r, g, b)
    }
}
