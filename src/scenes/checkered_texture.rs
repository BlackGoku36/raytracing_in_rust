use crate::raytrace::{
    hitable_list::HitableList,
    material::Lambertian,
    sphere::Sphere,
    texture::{CheckerTexture, ConstantTexture},
    vec::Vec3,
};
use std::sync::Arc;

pub fn checkered_texture_scene() -> HitableList {
    let mut world = HitableList::new(2);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
        ))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(CheckerTexture::new(
            Box::new(ConstantTexture::new(Vec3::new(0.2, 0.3, 0.1))),
            Box::new(ConstantTexture::new(Vec3::new(0.9, 0.9, 0.9))),
        ))),
    )));
    world
}
