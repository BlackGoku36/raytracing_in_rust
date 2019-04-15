use crate::raytrace::{
    hitable_list::HitableList,
    material::{Dielectric, Lambertian, Metal},
    moving_sphere::Movingsphere,
    sphere::Sphere,
    texture::ConstantTexture,
    vec::{drand48, Vec3},
};

use std::sync::Arc;

pub fn random_scene() -> HitableList {
    let n = 500;
    let mut world = HitableList::new(n + 1);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.4, 0.4, 0.5,
        ))))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                            drand48() * drand48(),
                            drand48() * drand48(),
                            drand48() * drand48(),
                        ))))),
                    )));
                } else if choose_mat < 0.95 {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + drand48()),
                                0.5 * (1.0 + drand48()),
                                0.5 * (1.0 + drand48()),
                            ),
                            0.5 * (1.0 + drand48()),
                        )),
                    )));
                } else {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}

pub fn moving_random_scene() -> HitableList {
    let n = 500;
    let mut world = HitableList::new(n + 1);
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.4, 0.4, 0.5,
        ))))),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = drand48();
            let center = Vec3::new(a as f32 + 0.9 * drand48(), 0.2, b as f32 + 0.9 * drand48());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    world.add(Box::new(Movingsphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5 * drand48(), 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                            drand48() * drand48(),
                            drand48() * drand48(),
                            drand48() * drand48(),
                        ))))),
                    )));
                } else if choose_mat < 0.95 {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + drand48()),
                                0.5 * (1.0 + drand48()),
                                0.5 * (1.0 + drand48()),
                            ),
                            0.5 * (1.0 + drand48()),
                        )),
                    )));
                } else {
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.4, 0.2, 0.1,
        ))))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    )));

    world
}
