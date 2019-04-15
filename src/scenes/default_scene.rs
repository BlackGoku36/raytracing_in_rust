use crate::raytrace::{
    hitable_list::HitableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    texture::ConstantTexture,
    vec::Vec3,
};
use std::sync::Arc;

pub fn default_scene() -> HitableList {
    let mut world = HitableList::new(4);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.2, 0.2, 0.8,
        ))))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.8, 0.8, 0.0,
        ))))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Arc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.1)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Arc::new(Dielectric::new(1.5)),
    )));

    world
}
