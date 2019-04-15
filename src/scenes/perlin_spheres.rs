use crate::raytrace::{
    hitable_list::HitableList, material::Lambertian, sphere::Sphere, texture::NoiseTexture,
    vec::Vec3,
};
use std::sync::Arc;

pub fn perlin_spheres() -> HitableList {
    let mut world = HitableList::new(2);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(Box::new(NoiseTexture::new(3.0)))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Box::new(NoiseTexture::new(3.0)))),
    )));

    world
}
