use crate::raytrace::{
    material::{
        Diffuse_Light,
        Lambertian
    },
    texture::ConstantTexture,
    hitable_list::HitableList,
    vec::Vec3,
    rectangle::{
        XY,
        XZ,
        YZ,
        Flip_Normal
    }
};
use std::sync::Arc;

pub fn cornell_box() -> HitableList {
    let mut world = HitableList::new(4);
    let red = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.65, 0.05, 0.05)))));
    let green = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.12, 0.45, 0.15)))));
    let white = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73)))));
    let light = Arc::new(Diffuse_Light::new(Box::new(ConstantTexture::new(Vec3::new(15.0, 15.0, 15.0)))));
    world.add(
        Flip_Normal::new(Box::new(YZ::new(0.0, 555.0, 0.0, 555.0, 555.0,
        green)))
    );
    world.add(
        Box::new(YZ::new(0.0, 555.0, 0.0, 555.0, 0.0,
        red))
    );
    world.add(
        Box::new(XZ::new(213.0, 343.0, 227.0, 332.0, 554.0,
        light))
    );
    world.add(
       Flip_Normal::new(Box::new(XZ::new(0.0, 555.0, 0.0, 555.0, 555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))))))))
    );
    world.add(
        Box::new(XZ::new(0.0, 555.0, 0.0, 555.0, 0.0,
        white))
    );
    world.add(
       Flip_Normal::new(Box::new(XY::new(0.0, 555.0, 0.0, 555.0, 555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(0.73, 0.73, 0.73))))))))
    );
    world
}