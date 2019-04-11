use crate::raytrace::{
    material::Lambertian,
    texture::NoiseTexture,
    hitable_list::HitableList,
    vec::Vec3,
    sphere::Sphere
};
use std::sync::Arc;

pub fn perlin_spheres() -> HitableList {
    let mut world = HitableList::new(2);
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

    world
}