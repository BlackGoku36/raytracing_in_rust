use crate::raytrace::{
    hitable_list::HitableList, material::Lambertian, sphere::Sphere, texture::ImageTexture,
    texture::NoiseTexture, vec::Vec3,
};

use image;
use std::sync::Arc;

pub fn textured_spheres() -> HitableList {
    let mut world = HitableList::new(2);

    let image = image::open("earth.png").expect("Can't find image").to_rgb();
    let (nx, ny) = image.dimensions();
    let pixels = image.into_raw();
    let texture = ImageTexture::new(pixels, nx, ny);

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(Box::new(texture))),
    )));

    world
}
