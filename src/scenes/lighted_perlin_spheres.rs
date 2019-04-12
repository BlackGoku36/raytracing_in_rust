use crate::raytrace::{
    material::{
        Diffuse_Light,
        Lambertian
    },
    texture::{
        NoiseTexture,
        ConstantTexture
    },
    hitable_list::HitableList,
    vec::Vec3,
    sphere::Sphere,
    rectangle::Rectangle
};
use std::sync::Arc;

pub fn lightted_perlin_spheres() -> HitableList {
    let mut world = HitableList::new(4);
    world.add(
        Box::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 
        2.0, 
        Arc::new(Lambertian::new(Box::new(NoiseTexture::new())))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 
        1000.0, 
        Arc::new(Lambertian::new(Box::new(NoiseTexture::new())))))
    );
    world.add(
        Box::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 
        2.0, 
        Arc::new(Diffuse_Light::new(Box::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0)))))))
    );
    world.add(
        Box::new(Rectangle::new(3.0, 5.0, 1.0, 3.0, -2.0,
        Arc::new(Diffuse_Light::new(Box::new(ConstantTexture::new(Vec3::new(4.0, 4.0, 4.0)))))))
    );
    world
}