use crate::raytrace::{
    bvh::BVHNode,
    constant_medium::ConstantMedium,
    cube::{Cube, RotateY, Translate},
    hitable::Hitable,
    hitable_list::HitableList,
    material::{Dielectric, DiffuseLight, Lambertian, Metal},
    moving_sphere::Movingsphere,
    rectangle::XZ,
    sphere::Sphere,
    texture::{ConstantTexture, NoiseTexture, ImageTexture},
    vec::{drand48, Vec3},
};
use std::sync::Arc;
use image;

pub fn final_scene() -> HitableList {
    let mut world = HitableList::new(30);
    let mut boxlist: Vec<Box<Hitable>> = vec![];
    let mut boxlist2: Vec<Box<Hitable>> = vec![];

    let center = Vec3::new(400.0, 400.0, 200.0);

    let nb = 18;

    let image = image::open("earth.png").expect("Can't find image").to_rgb();
    let (nx, ny) = image.dimensions();
    let pixels = image.into_raw();
    let texture = ImageTexture::new(pixels, nx, ny);

    for i in 0..nb {
        for j in 0..nb {
            let i = i as f32;
            let j = j as f32;
            let w = 100.0;
            let x0 = -1000.0 + i * w;
            let z0 = -1000.0 + j * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (drand48() + 0.01);
            let z1 = z0 + w;
            boxlist.push(Box::new(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                    0.48, 0.83, 0.53,
                ))))),
            )));
        }
    }
    world.add(BVHNode::construct(boxlist, 0.0, 1.0));
    world.add(Box::new(XZ::new(
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
            Vec3::new(7.0, 7.0, 7.0),
        )))),
    )));
    world.add(Box::new(Movingsphere::new(
        center,
        center + Vec3::new(30.0, 0.0, 0.0),
        0.0,
        1.0,
        50.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.7, 0.3, 0.1,
        ))))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0)),
    )));
    let boundary = Box::new(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary);
    world.add(ConstantMedium::new(
        Box::new(Sphere::new(
            Vec3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectric::new(1.5)),
        )),
        0.2,
        Box::new(ConstantTexture::new(Vec3::new(0.2, 0.4, 0.9))),
    ));
    world.add(ConstantMedium::new(
        Box::new(Sphere::new(
            Vec3::new(0.0, 0.0, 0.0),
            5000.0,
            Arc::new(Dielectric::new(1.5)),
        )),
        0.0001,
        Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));
    world.add(Box::new(Sphere::new(
        Vec3::new(400.0, 200.0, 400.0),
        100.0,
        Arc::new(Lambertian::new(Box::new(texture))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(Box::new(NoiseTexture::new(0.1)))),
    )));
    let ns = 750;
    for _j in 0..ns {
        boxlist2.push(Box::new(Sphere::new(
            Vec3::new(165.0 * drand48(), 165.0 * drand48(), 165.0 * drand48()),
            10.0,
            Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                0.73, 0.73, 0.73,
            ))))),
        )));
    }
    world.add(Translate::new(
        RotateY::new(BVHNode::construct(boxlist2, 0.0, 1.0), 15.0),
        Vec3::new(-100.0, 270.0, 395.0),
    ));
    world
}
