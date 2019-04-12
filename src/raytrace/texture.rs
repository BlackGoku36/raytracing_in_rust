use super::vec::Vec3;
use super::perlin::Perlin;

pub trait Texture: Sync + Send{
    fn value(&self, u: f32, v: f32, p: Vec3)-> Vec3;
}

pub struct ConstantTexture{
    pub color: Vec3
}

pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>
}
pub struct NoiseTexture{
    noise: Perlin,
    scale: f32
}

impl ConstantTexture{
    pub fn new(color: Vec3)-> Self{
        ConstantTexture{ color }
    }
    pub fn from_rgb(r: f32, g: f32, b:f32 )-> Self{
        ConstantTexture{
            color: Vec3::new(r, g, b)
        }
    }
}

impl CheckerTexture{
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>)-> Box<Self>{
        Box::new(Self{
            odd,
            even
        })
    }
}

impl NoiseTexture{
    pub fn new()-> Self{
        NoiseTexture{
            noise: Perlin::new(),
            scale: 3.0,
        }
    }

    fn turb(&self, p: Vec3, depth: i32)-> f32{
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _i in 0..depth{
            accum += weight*self.noise.noise(temp_p);
            weight*=0.5;
            temp_p*=2.0;
        }
        accum.abs()
    }
}

impl Texture for ConstantTexture{
    fn value(&self, _u: f32, _v: f32, _p: Vec3)-> Vec3{
        self.color
    }
}

impl Texture for CheckerTexture{
    fn value(&self, u: f32, v: f32, p: Vec3)-> Vec3{
        let sines = (10.0*p.x()).sin() * (10.0*p.y()).sin()*(10.0*p.z()).sin();
        if sines < 0.0{
            self.odd.value(u, v, p)
        }else{
            self.even.value(u, v, p)
        }
    }
}

impl Texture for NoiseTexture{
    fn value(&self, _u: f32, _v: f32, p: Vec3)-> Vec3{
        Vec3::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.scale * p.z() + 10.0*self.turb(p, 7)).sin())
    }
}