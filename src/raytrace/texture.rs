use super::vec::Vec3;

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