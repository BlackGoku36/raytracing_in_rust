use crate::raytrace::{
    constant_medium::ConstantMedium,
    cube::{Cube, RotateY, Translate},
    hitable_list::HitableList,
    material::{DiffuseLight, Lambertian},
    rectangle::{FlipNormal, XY, XZ, YZ},
    texture::ConstantTexture,
    vec::Vec3,
};
use std::sync::Arc;

pub fn cornell_box() -> HitableList {
    let mut world = HitableList::new(8);
    let red = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let green = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let white = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let light = Arc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
        Vec3::new(15.0, 15.0, 15.0),
    ))));
    world.add(FlipNormal::new(Box::new(YZ::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    ))));
    world.add(Box::new(YZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Box::new(XZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    world.add(FlipNormal::new(Box::new(XZ::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.73, 0.73, 0.73,
        ))))),
    ))));
    world.add(Box::new(XZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    world.add(FlipNormal::new(Box::new(XY::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.73, 0.73, 0.73,
        ))))),
    ))));
    world.add(Translate::new(
        RotateY::new(
            Box::new(Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                    0.73, 0.73, 0.73,
                ))))),
            )),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    ));
    world.add(Translate::new(
        RotateY::new(
            Box::new(Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                    0.73, 0.73, 0.73,
                ))))),
            )),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    world
}

pub fn cornell_smoke() -> HitableList {
    let mut world = HitableList::new(8);
    let red = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.65, 0.05, 0.05,
    )))));
    let green = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.12, 0.45, 0.15,
    )))));
    let white = Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
        0.73, 0.73, 0.73,
    )))));
    let light = Arc::new(DiffuseLight::new(Box::new(ConstantTexture::new(
        Vec3::new(15.0, 15.0, 15.0),
    ))));
    world.add(FlipNormal::new(Box::new(YZ::new(
        0.0, 555.0, 0.0, 555.0, 555.0, green,
    ))));
    world.add(Box::new(YZ::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Box::new(XZ::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    world.add(FlipNormal::new(Box::new(XZ::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.73, 0.73, 0.73,
        ))))),
    ))));
    world.add(Box::new(XZ::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    world.add(FlipNormal::new(Box::new(XY::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
            0.73, 0.73, 0.73,
        ))))),
    ))));
    let b1 = Translate::new(
        RotateY::new(
            Box::new(Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                    0.73, 0.73, 0.73,
                ))))),
            )),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let b2 = Translate::new(
        RotateY::new(
            Box::new(Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                Arc::new(Lambertian::new(Box::new(ConstantTexture::new(Vec3::new(
                    0.73, 0.73, 0.73,
                ))))),
            )),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );
    world.add(ConstantMedium::new(
        b1,
        0.01,
        Box::new(ConstantTexture::new(Vec3::new(1.0, 1.0, 1.0))),
    ));
    world.add(ConstantMedium::new(
        b2,
        0.01,
        Box::new(ConstantTexture::new(Vec3::new(0.0, 0.0, 0.0))),
    ));
    world
}
